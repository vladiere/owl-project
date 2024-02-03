use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    pub WEB_FOLDER: String,
    pub DB_URL: String,
}

impl CoreConfig {
    fn load_from_env() -> lib_utils::envs::Result<CoreConfig> {
        Ok(CoreConfig {
            WEB_FOLDER: get_env("WEB_FOLDER")?,
            DB_URL: get_env("SERVICE_DB_URL")?,
        })
    }
}
