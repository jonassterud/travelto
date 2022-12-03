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

/// Keys
#[derive(Debug, Clone, Hash)]
pub struct Keys {
    kiwi_search: String,
    kiwi_multicity: String,
    kiwi_nomad: String,
    rapid_key: String,
    rapid_skyscanner_host: String,
}

impl Default for Keys {
    fn default() -> Self {
        Self::from_env().unwrap_or_else(|_| Self::empty())
    }
}

impl Keys {
    /// Create [`Keys`] with empty values.
    pub fn empty() -> Keys {
        Keys {
            kiwi_search: String::new(),
            kiwi_multicity: String::new(),
            kiwi_nomad: String::new(),
            rapid_key: String::new(),
            rapid_skyscanner_host: String::new(),
        }
    }

    /// Create [`Keys`] from environmental variables.
    pub fn from_env() -> Result<Keys> {
        dotenv::dotenv().ok();

        Ok(Keys {
            kiwi_search: std::env::var("KIWI_SEARCH")?,
            kiwi_multicity: std::env::var("KIWI_MULTICITY")?,
            kiwi_nomad: std::env::var("KIWI_NOMAD")?,
            rapid_key: std::env::var("RAPID_KEY")?,
            rapid_skyscanner_host: std::env::var("RAPID_SKYSCANNER_HOST")?,
        })
    }

    /// Get Kiwi search API key.
    pub fn get_kiwi_search_key(&self) -> &str {
        &self.kiwi_search
    }

    /// Get Kiwi multicity API key.
    pub fn get_kiwi_multicity_key(&self) -> &str {
        &self.kiwi_multicity
    }

    /// Get Kiwi nomad API key.
    pub fn get_kiwi_nomad_key(&self) -> &str {
        &self.kiwi_nomad
    }

    /// Get Rapid Skyscanner API key.
    pub fn get_rapid_key(&self) -> &str {
        &self.rapid_key
    }

    /// Get Rapid Skyscanner API host.
    pub fn get_rapid_skyscanner_host(&self) -> &str {
        &self.rapid_skyscanner_host
    }
}
