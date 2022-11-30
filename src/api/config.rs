use anyhow::Result;
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
    /// Departure date range in "dd/mm/yyyy" format.
    pub departure_date: (String, String),
    /// Return date range in "dd/mm/yyyy" format. Can be empty.
    pub return_date: (String, String),
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
    /// * `departure_date` - date range to depart between.
    /// * `return_date` - date range to return between.
    pub fn new(
        from: &str,
        to: &str,
        departure_date: (&str, &str),
        return_date: (&str, &str),
        adults: u32,
        children: u32,
        infants: u32,
    ) -> Result<SearchConfig> {
        Ok(SearchConfig {
            from: from.to_owned(),
            to: to.to_owned(),
            departure_date: (departure_date.0.to_owned(), departure_date.1.to_owned()),
            return_date: (return_date.0.to_owned(), return_date.1.to_owned()),
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
        }
    }

    /// Create a new [`Keys`] from environmental variables.
    pub fn from_env() -> Result<Keys> {
        dotenv::dotenv().ok();

        Ok(Keys {
            kiwi_search: std::env::var("KIWI_SEARCH")?,
            kiwi_multicity: std::env::var("KIWI_MULTICITY")?,
            kiwi_nomad: std::env::var("KIWI_NOMAD")?,
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
}
