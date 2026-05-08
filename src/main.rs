mod db;
mod handlers;
mod i18n;
mod models;
mod templates;

use anyhow::Result;
use axum::{routing::get, Router};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::{SqliteStore};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_ci=debug,tower_sessions=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Init DB
    let pool = db::init_db().await?;

    // Setup session store
    let session_store = SqliteStore::new(pool.clone());
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set true if using HTTPS
        .with_expiry(Expiry::OnInactivity(time::Duration::days(1)));

    let state = AppState { db: pool };

    let app = Router::new()
        .merge(handlers::auth::router())
        .merge(handlers::projects::router())
        .merge(handlers::project_logs::router())
        .merge(handlers::builds::router())
        .merge(handlers::webhook::router())
        .layer(session_layer)
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valid number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    deletion_task.abort();
    Ok(())
}
