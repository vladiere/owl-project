mod admin;
mod auth;
pub mod extract_token;

pub use admin::*;
pub use auth::*;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Deserialize, sqlx::Type, Serialize, Clone)]
#[sqlx(type_name = "user_type")]
pub enum UserType {
    Admin,
    Super,
    User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthAdmin {
    id: i64,
    username: String,
    password: String,
    token_salt: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleUser {
    role_user: UserType,
}

impl FromRow<'_, PgRow> for RoleUser {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            role_user: row.try_get(0)?,
            // Map other fields...
        })
    }
}

impl FromRow<'_, PgRow> for AuthAdmin {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(0)?,
            username: row.try_get(1)?,
            password: row.try_get(2)?,
            token_salt: row.try_get(3)?,
            // Map other fields...
        })
    }
}
