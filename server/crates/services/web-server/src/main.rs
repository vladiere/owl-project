use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod error;
mod web;

pub use self::error::{Error, Result};

use crate::web::routes_auth::routes;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = Router::new()
        .merge(routes())
        .route("/hello", get(hello_handler));

    let host_addr = "127.0.0.1:4000";

    let tcp_listener = tokio::net::TcpListener::bind(host_addr).await.unwrap();
    info!("{:<12} - http://{host_addr}", "LISTENING");

    axum::serve(tcp_listener, app).await.unwrap();

    Ok(())
}

async fn hello_handler() -> impl IntoResponse {
    info!("{:<12} - hello_handler", "HANDLER");

    Html("Success OK.")
}
