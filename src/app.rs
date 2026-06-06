use crate::{api, handlers, state::AppState};
use axum::Router;
use moka::future::Cache;
use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};
use std::{env, error::Error, fs, net::SocketAddr, path::PathBuf};
use tower_http::trace::TraceLayer;

pub async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".to_string());

    let data_dir = PathBuf::from("data");
    fs::create_dir_all(&data_dir)?;

    let db_path = data_dir.join("shortener.db");

    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let cache = Cache::builder()
        .max_capacity(100_000)
        .time_to_live(std::time::Duration::from_secs(60 * 60))
        .build();

    let state = AppState { pool, base_url, cache };

    let app = router(state).layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn router(state: AppState) -> Router {
    use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
    use std::sync::Arc;

    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .unwrap(),
    );

    let governor_layer = tower_governor::GovernorLayer::new(governor_conf);

    Router::new()
        .route("/", axum::routing::get(handlers::home))
        .route("/healthz", axum::routing::get(handlers::healthz))
        .route(
            "/shorten",
            axum::routing::post(handlers::shorten).layer(governor_layer.clone()),
        )
        .route("/stats/{code}", axum::routing::get(handlers::stats))
        .route("/api/v1/links", axum::routing::post(api::create_link).layer(governor_layer))
        .route("/api/v1/links/{code}", axum::routing::get(api::get_link))
        .route("/qr/{code}", axum::routing::get(handlers::qr_code))
        .route("/{code}", axum::routing::get(handlers::redirect_short_link))
        .with_state(state)
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
