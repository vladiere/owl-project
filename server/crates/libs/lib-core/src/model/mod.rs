pub mod error;
pub mod store;

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

// endregion: ---- Modules

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    // ModelManager Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    // Returns the sqlx db pool reference.
    /// (Only for the model layer)
    pub fn db(&self) -> &Db {
        &self.db
    }
}
