use super::Keys;
use anyhow::Result;
use serde::Deserialize;
use std::hash::Hash;

/// Configuration used for location search APIs.
#[derive(Debug, Clone, Deserialize, Hash)]
pub struct LocationConfig {
    /// Term to search for.
    pub term: String,
    /// API keys.
    #[serde(skip)]
    pub keys: Keys,
}

impl LocationConfig {
    /// Create a new [`LocationConfig`].
    ///
    /// # Arguments
    ///
    /// * `term` - term to search for.
    pub fn new(term: &str) -> Result<LocationConfig> {
        Ok(LocationConfig {
            term: term.to_owned(),
            keys: Keys::from_env()?,
        })
    }
}
