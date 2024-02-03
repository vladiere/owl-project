// region: ---- Modules
use base64::engine::{general_purpose, Engine};

// endregion: ---- Modules

// region: ---- Public functions

pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(content)
}

pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>> {
    general_purpose::URL_SAFE_NO_PAD
        .decode(b64u)
        .map_err(|_| Error::FailToB64Decode)
}

pub fn b64u_decode_to_string(b64u: &str) -> Result<String> {
    b64u_decode(b64u)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(Error::FailToB64Decode)
}

// endregion: ---- Public functions

// region: ---- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailToB64Decode,
}

// endregion: ---- Error

// region: ---- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: ---- Error Boilerplate
