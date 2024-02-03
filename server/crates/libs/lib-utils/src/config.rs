use crate::envs::{get_env, Result};
use std::sync::OnceLock;

// region: ---- Public function

pub fn utils_config() -> &'static UtilsConfig {
    static INSTANCE: OnceLock<UtilsConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        UtilsConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

// endregion: ---- Public function

// region: ---- Public structs

#[allow(non_snake_case)]
pub struct UtilsConfig {
    // -- Hash secret
    pub HASH_SECRET: String,
}

// endregion: ---- Public structs

// region: ---- Struct impl

impl UtilsConfig {
    fn load_from_env() -> Result<UtilsConfig> {
        Ok(UtilsConfig {
            // -- Hash secret
            HASH_SECRET: get_env("HASH_SECRET")?,
        })
    }
}

// endregion: ---- Struct impl
