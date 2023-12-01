use std::fmt;

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    FetchError(String),
    JsonError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FetchError(s) => write!(f, "{}", s),
            Self::JsonError(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for AppError {}
