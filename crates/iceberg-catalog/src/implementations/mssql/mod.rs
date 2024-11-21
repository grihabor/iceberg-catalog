mod catalog;
use anyhow::Context;
pub(crate) mod dbutils;
mod error;
pub(crate) mod namespace;
mod pagination;
mod pool;
pub(crate) mod secrets;
pub(crate) mod warehouse;

use self::dbutils::DBErrorHandler;
use crate::api::Result;
use crate::config::{DynAppConfig, PgSslMode};
use crate::service::health::{Health, HealthExt, HealthStatus};
use crate::CONFIG;
use anyhow::anyhow;
use async_trait::async_trait;
use deadpool::{
    managed,
    managed::{Hook, HookFuture, HookResult, Metrics, PoolConfig, RecycleError, RecycleResult},
    Runtime,
};
use pool::Manager;
pub use secrets::SecretsState as SecretsStore;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

/// # Errors
/// Returns an error if the pool creation fails.
pub async fn get_reader_pool(
    mut config: tiberius::Config,
    pool_config: PoolConfig,
) -> anyhow::Result<pool::Pool> {
    config.readonly(true);
    let manager = Manager::new(config, pool_config);
    let pool = manager
        .create_pool()
        .map_err(|e| anyhow::anyhow!(e).context("Error creating read pool."))?;
    Ok(pool)
}

/// # Errors
/// Returns an error if the pool cannot be created.
pub async fn get_writer_pool(
    config: tiberius::Config,
    pool_config: PoolConfig,
) -> anyhow::Result<pool::Pool> {
    let pool = Manager::new(config, pool_config)
        .create_pool()
        .map_err(|e| anyhow::anyhow!(e).context("Error creating write pool."))?;
    Ok(pool)
}

/// # Errors
/// Returns an error if the migration fails.
pub async fn migrate(pool: &pool::Pool) -> anyhow::Result<()> {
    todo!()
}

/// # Errors
/// Returns an error if db connection fails or if migrations are missing.
pub async fn check_migration_status(pool: &pool::Pool) -> anyhow::Result<MigrationState> {
    todo!()
}

#[derive(Debug, Copy, Clone)]
pub enum MigrationState {
    Complete,
    Missing,
    NoMigrationsTable,
}

#[derive(Debug, Clone)]
pub struct Catalog {}

#[derive(Debug)]

pub struct MssqlTransaction {
    conn: deadpool::managed::Object<pool::Manager>,
}

#[async_trait::async_trait]
impl crate::service::Transaction<CatalogState> for MssqlTransaction {
    type Transaction<'a> = &'a mut sqlx::Transaction<'static, sqlx::Postgres>;

    async fn begin_write(db_state: CatalogState) -> Result<Self> {
        let mut conn = db_state
            .write_pool()
            .get()
            .await
            .map_err(|e| e.into_error_model("Error starting transaction".to_string()))?;
        conn.simple_query("BEGIN TRANSACTION")
            .await
            .map_err(|e| e.into_error_model("Error starting transaction".to_string()))?;

        Ok(Self { conn })
    }

    async fn begin_read(db_state: CatalogState) -> Result<Self> {
        let mut conn = db_state
            .read_pool()
            .get()
            .await
            .map_err(|e| e.into_error_model("Error starting transaction".to_string()))?;
        conn.simple_query("BEGIN TRANSACTION")
            .await
            .map_err(|e| e.into_error_model("Error starting transaction".to_string()))?;

        Ok(Self { conn })
    }

    async fn commit(mut self) -> Result<()> {
        self.conn
            .simple_query("COMMIT")
            .await
            .map_err(|e| e.into_error_model("Error committing transaction".to_string()))?;
        Ok(())
    }

    async fn rollback(mut self) -> Result<()> {
        self.conn
            .simple_query("ROLLBACK")
            .await
            .map_err(|e| e.into_error_model("Error rolling back transaction".to_string()))?;
        Ok(())
    }

    fn transaction(&mut self) -> Self::Transaction<'_> {
        &mut self.transaction
    }
}

#[derive(Clone, Debug)]
pub struct ReadWrite {
    pub read_pool: pool::Pool,
    pub write_pool: pool::Pool,
    pub health: Arc<RwLock<Vec<Health>>>,
}

#[async_trait]
impl HealthExt for ReadWrite {
    async fn health(&self) -> Vec<Health> {
        self.health.read().await.clone()
    }

    async fn update_health(&self) {
        let read = self.read_health().await;
        let write = self.write_health().await;
        let mut lock = self.health.write().await;
        lock.clear();
        lock.extend([
            Health::now("read_pool", read),
            Health::now("write_pool", write),
        ]);
    }
}

impl ReadWrite {
    #[must_use]
    pub fn from_pools(read_pool: pool::Pool, write_pool: pool::Pool) -> Self {
        Self {
            read_pool,
            write_pool,
            health: Arc::new(RwLock::new(vec![
                Health::now("read_pool", HealthStatus::Unknown),
                Health::now("write_pool", HealthStatus::Unknown),
            ])),
        }
    }

    async fn health(pool: pool::Pool) -> HealthStatus {
        match Self::_health(pool).await {
            Ok(_) => HealthStatus::Healthy,
            Err(e) => {
                tracing::warn!(?e, "Pool is unhealthy");
                return HealthStatus::Unhealthy;
            }
        }
    }

    async fn _health(pool: pool::Pool) -> anyhow::Result<()> {
        let mut conn = pool.get().await?;
        conn.simple_query("SELECT 1").await?.into_row().await?;
        Ok(())
    }

    async fn write_health(&self) -> HealthStatus {
        Self::health(self.write_pool.clone()).await
    }

    async fn read_health(&self) -> HealthStatus {
        Self::health(self.read_pool.clone()).await
    }
}

#[derive(Clone, Debug)]

pub struct CatalogState {
    pub read_write: ReadWrite,
}

#[async_trait]
impl HealthExt for CatalogState {
    async fn health(&self) -> Vec<Health> {
        self.read_write.health().await
    }

    async fn update_health(&self) {
        self.read_write.update_health().await;
    }
}

impl CatalogState {
    #[must_use]
    pub fn from_pools(read_pool: pool::Pool, write_pool: pool::Pool) -> Self {
        Self {
            read_write: ReadWrite::from_pools(read_pool, write_pool),
        }
    }

    #[must_use]
    pub fn read_pool(&self) -> pool::Pool {
        self.read_write.read_pool.clone()
    }

    #[must_use]
    pub fn write_pool(&self) -> pool::Pool {
        self.read_write.write_pool.clone()
    }
}
pub use secrets::SecretsState;

impl DynAppConfig {
    pub fn mssql_config(&self) -> anyhow::Result<(tiberius::Config, PoolConfig)> {
        let connection_string = self
            .mssql_jdbc_connection_string
            .clone()
            .ok_or(anyhow!("mssql_jdbc_connection_string is not set"))?;
        let config = tiberius::Config::from_jdbc_string(&connection_string)
            .context("valid jdbc connection string")?;
        Ok((config, PoolConfig::default()))
    }
}

#[derive(Debug, Clone, Copy)]
enum ConnectionType {
    Read,
    Write,
}
