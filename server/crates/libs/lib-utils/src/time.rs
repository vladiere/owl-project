use time::{Duration, OffsetDateTime};

// region: ---- Public functions

pub use time::format_description::well_known::Rfc3339;

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

pub fn format_time(time: OffsetDateTime) -> String {
    time.format(&Rfc3339).unwrap() // TODO: need to check if safe
}

pub fn now_utc_plus_sec_str(sec: f64) -> String {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339)
        .map_err(|_| Error::FailToDateParse(moment.to_string()))
}

// endregion: ---- Public functions

// region: ---- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailToDateParse(String),
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

// endregion: ---- Error Boilerplate
