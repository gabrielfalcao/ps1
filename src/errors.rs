use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    IOError(String),
    ConfigError(String),
    SafetyError(String),
    ParseError(String),
    SyntaxTableError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Self::IOError(s) => format!("{}", s),
                Self::ConfigError(s) => format!("{}", s),
                Self::SafetyError(s) => format!("{}", s),
                Self::ParseError(s) => format!("{}", s),
                Self::SyntaxTableError(s) => format!("{}", s),
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::IOError(_) => "IOError",
            Error::ConfigError(_) => "ConfigError",
            Error::SafetyError(_) => "SafetyError",
            Error::ParseError(_) => "ParseError",
            Error::SyntaxTableError(_) => "SyntaxTableError",
        }
        .to_string()
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<iocore::Error> for Error {
    fn from(e: iocore::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}
impl From<toml::ser::Error> for Error {
    fn from(e: toml::ser::Error) -> Self {
        Error::ConfigError(format!("{}", e))
    }
}
impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::ConfigError(format!("{}", e))
    }
}
impl From<sanitation::Error<'_>> for Error {
    fn from(e: sanitation::Error<'_>) -> Self {
        Error::SafetyError(format!("{}", e))
    }
}
pub type Result<T> = std::result::Result<T, Error>;
