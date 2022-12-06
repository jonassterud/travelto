use super::Keys;
use anyhow::Result;
use chrono::NaiveDate;
use serde::Deserialize;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// Configuration used for search APIs.
#[derive(Debug, Clone, Deserialize, Hash)]
pub struct SearchConfig {
    /// Place to travel from.
    pub from: String,
    /// Place to travel to.
    pub to: String,
    /// Start of departure date range.
    pub departure_from: NaiveDate,
    /// Start of departure date range.
    pub departure_to: Option<NaiveDate>,
    /// Start of return date range.
    pub return_from: Option<NaiveDate>,
    /// End of return date range.
    pub return_to: Option<NaiveDate>,
    /// Amount of adults.
    pub adults: u32,
    /// Amount of children.
    pub children: u32,
    /// Amount of infants.
    pub infants: u32,
    /// API keys.
    #[serde(skip)]
    pub keys: Keys,
}

impl SearchConfig {
    /// Create a new [`SearchConfig`].
    ///
    /// # Arguments
    ///
    /// * `from` - place to travel from.
    /// * `to` - place to travel to.
    /// * `departure_from` - start of date range to depart between.
    /// * `departure_to` - end of date range to depart between.
    /// * `return_from` - start of date range to return between.
    /// * `return_to` - end of date range to return between.
    /// * `adults` - number of adults.
    /// * `children` - number of children.
    /// * `infants` - number of infants.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        from: String,
        to: String,
        departure_from: NaiveDate,
        departure_to: Option<NaiveDate>,
        return_from: Option<NaiveDate>,
        return_to: Option<NaiveDate>,
        adults: u32,
        children: u32,
        infants: u32,
    ) -> Result<SearchConfig> {
        Ok(SearchConfig {
            from,
            to,
            departure_from,
            departure_to,
            return_from,
            return_to,
            adults,
            children,
            infants,
            keys: Keys::from_env()?,
        })
    }

    /// Get hash.
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
