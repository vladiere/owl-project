use argonautica::Hasher;
use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use lib_core::model::{store::new_db_pool, ModelManager, Result};
use lib_utils::utils_config;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tracing::debug;

pub fn routes() -> Router {
    Router::new()
        .route("/admin/register", post(admin_register))
        .route("/admin/test", get(admin_test))
}

async fn admin_register(
    Json(payload): Json<LoginPayload>,
) -> Result<Json<AdminReturn>> {
    debug!("{:<12} - admin_register", "HANDLER");

    let db = new_db_pool().await?;
    let query = "insert into admin_table (username, password) values ($1,$2) returning id, pass_salt, token_salt";

    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(payload.password)
        .with_secret_key(&utils_config().HASH_SECRET)
        .hash()
        .unwrap();

    let res = sqlx::query_as::<_, AdminReturn>(query)
        .bind(payload.username)
        .bind(hash)
        .fetch_one(&db)
        .await?;

    Ok(Json(res))
}

async fn admin_test() -> impl IntoResponse {
    debug!("{:<12} - admin_test", "TESTING HANDLER");

    let res = TestPayload {
        message: "testing ok".to_string(),
    };

    Json(res)
}

#[derive(Debug, Deserialize, Serialize)]
struct TestPayload {
    message: String,
}

#[derive(Serialize, FromRow)]
pub struct AdminReturn {
    pub id: i32,
    pub pass_salt: String,
    pub token_salt: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
