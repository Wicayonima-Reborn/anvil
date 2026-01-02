use super::ConfigError;

pub fn required(key: &'static str) -> Result<String, ConfigError> {
    std::env::var(key).map_err(|_| ConfigError::MissingVar(key))
}

pub fn optional(key: &'static str) -> Option<String> {
    std::env::var(key).ok()
}

pub fn required_parse<T>(
    key: &'static str,
) -> Result<T, ConfigError>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let raw = required(key)?;
    raw.parse::<T>().map_err(|e| ConfigError::InvalidVar {
        key,
        reason: e.to_string(),
    })
}