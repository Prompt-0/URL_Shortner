use moka::future::Cache;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub base_url: String,
    pub cache: Cache<String, crate::models::LinkRecord>,
}
