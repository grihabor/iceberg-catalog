use super::{
    namespace::{
        create_namespace, drop_namespace, get_namespace, list_namespaces, namespace_ident_to_id,
        update_namespace_properties,
    },
    warehouse::{
        create_warehouse, delete_warehouse, get_warehouse, list_projects, list_warehouses,
        rename_warehouse, set_warehouse_status, update_storage_profile,
    },
    CatalogState, PostgresTransaction,
};
use crate::service::ViewMetadataWithLocation;
use crate::service::{
    CreateNamespaceRequest, CreateNamespaceResponse, CreateTableRequest, GetWarehouseResponse,
    ListNamespacesQuery, ListNamespacesResponse, NamespaceIdent, Result, TableIdent,
    WarehouseStatus,
};
use crate::{
    api::iceberg::v1::{PaginatedTabulars, PaginationQuery},
    service::TableCommit,
};
use crate::{
    service::{
        storage::StorageProfile, Catalog, CreateTableResponse, GetNamespaceResponse,
        GetTableMetadataResponse, LoadTableResponse, NamespaceIdentUuid, ProjectIdent,
        TableIdentUuid, Transaction, WarehouseIdent,
    },
    SecretIdent,
};
use iceberg::spec::ViewMetadata;
use std::collections::{HashMap, HashSet};

#[async_trait::async_trait]
impl Catalog for super::Catalog {
    type Transaction = PostgresTransaction;
    type State = CatalogState;

    async fn create_warehouse<'a>(
        warehouse_name: String,
        project_id: ProjectIdent,
        storage_profile: StorageProfile,
        storage_secret_id: Option<SecretIdent>,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<WarehouseIdent> {
        todo!();
    }

    async fn get_warehouse<'a>(
        warehouse_id: WarehouseIdent,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<GetWarehouseResponse> {
        todo!();
    }

    async fn get_namespace<'a>(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<GetNamespaceResponse> {
        todo!();
    }

    async fn list_namespaces(
        warehouse_id: WarehouseIdent,
        query: &ListNamespacesQuery,
        catalog_state: CatalogState,
    ) -> Result<ListNamespacesResponse> {
        todo!();
    }

    async fn create_namespace<'a>(
        warehouse_id: WarehouseIdent,
        namespace_id: NamespaceIdentUuid,
        request: CreateNamespaceRequest,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<CreateNamespaceResponse> {
        todo!();
    }

    async fn namespace_ident_to_id(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        catalog_state: CatalogState,
    ) -> Result<Option<NamespaceIdentUuid>> {
        todo!();
    }

    async fn drop_namespace<'a>(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<()> {
        todo!();
    }

    async fn update_namespace_properties<'a>(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        properties: HashMap<String, String>,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<()> {
        todo!();
    }

    async fn create_table<'a>(
        namespace_id: NamespaceIdentUuid,
        table: &TableIdent,
        table_id: TableIdentUuid,
        request: CreateTableRequest,
        // Metadata location may be none if stage-create is true
        metadata_location: Option<&str>,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<CreateTableResponse> {
        todo!();
    }

    async fn list_tables(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        list_flags: crate::service::ListFlags,
        catalog_state: CatalogState,
        pagination_query: PaginationQuery,
    ) -> Result<PaginatedTabulars<TableIdentUuid, TableIdent>> {
        todo!();
    }

    // Should also load staged tables but not tables of inactive warehouses
    async fn load_tables<'a>(
        warehouse_id: WarehouseIdent,
        tables: impl IntoIterator<Item = TableIdentUuid> + Send,
        include_deleted: bool,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<HashMap<TableIdentUuid, LoadTableResponse>> {
        todo!();
    }

    async fn get_table_metadata_by_id(
        warehouse_id: WarehouseIdent,
        table: TableIdentUuid,
        list_flags: crate::service::ListFlags,
        catalog_state: Self::State,
    ) -> Result<GetTableMetadataResponse> {
        todo!();
    }

    async fn get_table_metadata_by_s3_location(
        warehouse_id: WarehouseIdent,
        location: &str,
        list_flags: crate::service::ListFlags,
        catalog_state: Self::State,
    ) -> Result<GetTableMetadataResponse> {
        todo!();
    }

    async fn table_ident_to_id(
        warehouse_id: WarehouseIdent,
        table: &TableIdent,
        list_flags: crate::service::ListFlags,
        catalog_state: Self::State,
    ) -> Result<Option<TableIdentUuid>> {
        todo!();
    }

    async fn rename_table<'a>(
        warehouse_id: WarehouseIdent,
        source_id: TableIdentUuid,
        source: &TableIdent,
        destination: &TableIdent,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<()> {
        todo!();
    }

    async fn drop_table<'a>(
        table_id: TableIdentUuid,
        hard_delete: bool,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()> {
        todo!();
    }

    async fn table_idents_to_ids(
        warehouse_id: WarehouseIdent,
        tables: HashSet<&TableIdent>,
        list_flags: crate::service::ListFlags,
        catalog_state: Self::State,
    ) -> Result<HashMap<TableIdent, Option<TableIdentUuid>>> {
        todo!();
    }

    async fn commit_table_transaction<'a>(
        warehouse_id: WarehouseIdent,
        commits: impl IntoIterator<Item = TableCommit> + Send,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<()> {
        todo!();
    }

    // ---------------- Management API ----------------
    async fn list_projects(catalog_state: Self::State) -> Result<HashSet<ProjectIdent>> {
        todo!();
    }

    async fn list_warehouses(
        project_id: ProjectIdent,
        include_inactive: Option<Vec<WarehouseStatus>>,
        warehouse_id_filter: Option<&HashSet<WarehouseIdent>>,
        catalog_state: Self::State,
    ) -> Result<Vec<GetWarehouseResponse>> {
        todo!();
    }

    async fn delete_warehouse<'a>(
        warehouse_id: WarehouseIdent,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<()> {
        todo!();
    }

    async fn rename_warehouse<'a>(
        warehouse_id: WarehouseIdent,
        new_name: &str,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<()> {
        todo!();
    }

    async fn set_warehouse_status<'a>(
        warehouse_id: WarehouseIdent,
        status: WarehouseStatus,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<()> {
        todo!();
    }

    async fn update_storage_profile<'a>(
        warehouse_id: WarehouseIdent,
        storage_profile: StorageProfile,
        storage_secret_id: Option<SecretIdent>,
        transaction: <Self::Transaction as Transaction<CatalogState>>::Transaction<'a>,
    ) -> Result<()> {
        todo!();
    }

    async fn create_view<'a>(
        namespace_id: NamespaceIdentUuid,
        view: &TableIdent,
        request: ViewMetadata,
        metadata_location: &str,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()> {
        todo!();
    }

    async fn view_ident_to_id(
        warehouse_id: WarehouseIdent,
        view: &TableIdent,
        catalog_state: Self::State,
    ) -> Result<Option<TableIdentUuid>> {
        todo!();
    }

    async fn load_view<'a>(
        view_id: TableIdentUuid,
        include_deleted: bool,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<ViewMetadataWithLocation> {
        todo!();
    }

    async fn list_views(
        warehouse_id: WarehouseIdent,
        namespace: &NamespaceIdent,
        include_deleted: bool,
        catalog_state: Self::State,
        pagination_query: PaginationQuery,
    ) -> Result<PaginatedTabulars<TableIdentUuid, TableIdent>> {
        todo!();
    }

    async fn update_view_metadata(
        namespace_id: NamespaceIdentUuid,
        view_id: TableIdentUuid,
        view: &TableIdent,
        metadata_location: &str,
        metadata: ViewMetadata,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'_>,
    ) -> Result<()> {
        todo!();
    }

    async fn rename_view(
        warehouse_id: WarehouseIdent,
        source_id: TableIdentUuid,
        source: &TableIdent,
        destination: &TableIdent,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'_>,
    ) -> Result<()> {
        todo!();
    }

    async fn drop_view<'a>(
        table_id: TableIdentUuid,
        transaction: <Self::Transaction as Transaction<Self::State>>::Transaction<'a>,
    ) -> Result<()> {
        todo!();
    }
}
