use lib_utils::envs::get_env;
use std::sync::OnceLock;

pub fn web_config() -> &'static WebConfig {
    static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        WebConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
pub struct WebConfig {
    pub WEB_FOLDER: String,
    pub DB_URL: String,
}

impl WebConfig {
    fn load_from_env() -> lib_utils::envs::Result<WebConfig> {
        Ok(WebConfig {
            WEB_FOLDER: get_env("WEB_FOLDER")?,
            DB_URL: get_env("SERVICE_DB_URL")?,
        })
    }
}
