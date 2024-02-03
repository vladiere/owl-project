mod error;

pub use self::error::{Error, Result};

use axum::{routing::get, Json, Router};
use serde::Serialize;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time() // For early local development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = Router::new().route("/", get(default_route));
    let host_addr = "127.0.0.1:4000";
    let tcp_listener = tokio::net::TcpListener::bind(host_addr).await.unwrap();
    info!("{:<12} - http://{host_addr}", "SERVER LISTENING");
    axum::serve(tcp_listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Serialize)]
struct DefaultRoute {
    message: &'static str,
}

async fn default_route() -> Json<DefaultRoute> {
    debug!("{:<12} = default_route", "HANDLER");

    Json(DefaultRoute { message: "Success" })
}
