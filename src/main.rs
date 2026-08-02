mod app;
pub mod auth;
pub mod error;
pub mod models;
pub mod repository;
pub mod routes;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    app::App::start().await
}
