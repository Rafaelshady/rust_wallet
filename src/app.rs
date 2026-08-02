use crate::routes;
use axum::Router;
use color_eyre::eyre::Ok;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt,
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub usd_brl_rate: f64,
}

impl AppState {
    async fn new() -> color_eyre::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let usd_brl_rate = std::env::var("USD_BRL_RATE")
            .unwrap_or_else(|_| "5.50".to_string())
            .parse::<f64>()?;

        if !usd_brl_rate.is_finite() || usd_brl_rate <= 0.0 {
            return Err(color_eyre::eyre::eyre!(
                "USD_BRL_RATE must be a positive number"
            ));
        }

        let db = PgPool::connect(&database_url).await?;
        Ok(Self { db, usd_brl_rate })
    }
}

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        dotenvy::dotenv()?;
        let state = AppState::new().await?;

        let listener = TcpListener::bind("0.0.0.0:3000").await?;
        let router = Router::new()
            .nest("/api", routes::api::router())
            .merge(routes::frontend::router())
            .with_state(state);

        info!("Starting service");

        axum::serve(listener, router).await?;

        Ok(())
    }
}
