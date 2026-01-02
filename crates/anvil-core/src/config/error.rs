use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    MissingVar(&'static str),
    InvalidVar {
        key: &'static str,
        reason: String,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingVar(key) => {
                write!(f, "Missing required environment variable: {}", key)
            }
            ConfigError::InvalidVar { key, reason } => {
                write!(f, "Invalid value for {}: {}", key, reason)
            }
        }
    }
}

impl std::error::Error for ConfigError {}