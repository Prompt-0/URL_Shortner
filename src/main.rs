mod api;
mod app;
mod db;
mod error;
mod handlers;
mod models;
mod services;
mod state;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    app::run().await
}
