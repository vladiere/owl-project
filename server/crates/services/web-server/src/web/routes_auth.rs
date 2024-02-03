use argonautica::Hasher;
use axum::{routing::post, Json, Router};
use lib_core::model::{error::Result, store::new_db_pool};
use lib_utils::utils_config;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tracing::debug;

pub fn routes() -> Router {
    Router::new().route("/api/admin/register", post(admin_register))
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
