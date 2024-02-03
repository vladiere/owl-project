use argonautica::Hasher;

use crate::utils_config;

pub fn encrypt_pass(pass: String) -> Result<String> {
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(pass)
        .with_secret_key(&utils_config().HASH_SECRET)
        .hash()
        .unwrap();

    Ok(hash.to_string())
}

// region: ---- Error
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailedToHashPassword,
}
// endregion: ---- Error

// region: ---- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: ---- Error Boilerplate

// region: ---- Testing

#[cfg(test)]
mod tests {
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    use super::encrypt_pass;

    #[test]
    fn test_encrypt_pass_ok() -> Result<()> {
        let pass = "hello world".to_string();
        let fx_res = "$argon2id$v=19$m=4096,t=192,p=4$4JVgBqALZfNIApF2IeWO5WBYxPW5uIeojyVIeDuthhw$XNZ5d
4wTPpdyhUdS93m+0ja+QixMAsunb+TaxF8Tpb4";
        let hash = encrypt_pass(pass)?;

        assert_eq!(fx_res, hash);

        Ok(())
    }
}

// endregion: ---- Testing
