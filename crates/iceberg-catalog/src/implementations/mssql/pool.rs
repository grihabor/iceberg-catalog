use std::{borrow::Cow, mem::replace};

use deadpool::{
    managed,
    managed::{Hook, HookFuture, HookResult, Metrics, PoolConfig, RecycleError, RecycleResult},
    Runtime,
};
use tiberius::error::Error;
use tiberius::{AuthMethod, EncryptionLevel};
use tokio_util::compat::TokioAsyncWriteCompatExt;

use super::error::SqlServerError;

/// Type aliasing for tiberius client with [`tokio`] as runtime.
pub(super) type Client = tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>;
/// Type aliasing for Pool.
pub(super) type Pool = managed::Pool<Manager>;

/// Copied from deadpool-tiberius.
#[derive(Debug)]
pub(super) struct Manager {
    config: tiberius::Config,
    pool_config: PoolConfig,
    runtime: Option<Runtime>,
    hooks: Hooks,
}

impl managed::Manager for Manager {
    type Type = Client;
    type Error = tiberius::error::Error;

    async fn create(&self) -> Result<Client, Self::Error> {
        let tcp = tokio::net::TcpStream::connect(self.config.get_addr()).await?;
        let client = Client::connect(self.config.clone(), tcp.compat_write()).await;

        match client {
            Ok(client) => Ok(client),
            Err(Error::Routing { host, port }) => {
                let mut config = self.config.clone();
                config.host(host);
                config.port(port);

                let tcp = tokio::net::TcpStream::connect(config.get_addr()).await?;
                tcp.set_nodelay(true)?;

                Client::connect(config, tcp.compat_write()).await
            }
            // Propagate errors
            Err(err) => Err(err)?,
        }
    }

    async fn recycle(
        &self,
        obj: &mut Self::Type,
        _metrics: &Metrics,
    ) -> RecycleResult<Self::Error> {
        match obj.simple_query("").await {
            Ok(_) => Ok(()),
            Err(e) => Err(RecycleError::Message(Cow::from(e.to_string()))),
        }
    }
}

impl Manager {
    pub(super) fn new(config: tiberius::Config, pool_config: PoolConfig) -> Manager {
        Manager {
            config,
            pool_config,
            runtime: None,
            hooks: Default::default(),
        }
    }

    /// Consume self, build a pool.
    pub fn create_pool(mut self) -> Result<Pool, SqlServerError> {
        let config = self.pool_config;
        let runtime = self.runtime;
        let hooks = replace(&mut self.hooks, Hooks::default());
        let mut pool = Pool::builder(self).config(config);
        if let Some(v) = runtime {
            pool = pool.runtime(v);
        }

        for hook in hooks.post_create {
            pool = pool.post_create(hook);
        }
        for hook in hooks.pre_recycle {
            pool = pool.pre_recycle(hook);
        }
        for hook in hooks.post_recycle {
            pool = pool.post_recycle(hook);
        }

        Ok(pool.build()?)
    }
}

struct Hooks {
    pre_recycle: Vec<Hook<Manager>>,
    post_recycle: Vec<Hook<Manager>>,
    post_create: Vec<Hook<Manager>>,
}

impl Default for Hooks {
    fn default() -> Self {
        Hooks {
            pre_recycle: Vec::<Hook<Manager>>::new(),
            post_recycle: Vec::<Hook<Manager>>::new(),
            post_create: Vec::<Hook<Manager>>::new(),
        }
    }
}
