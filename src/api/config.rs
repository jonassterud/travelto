use anyhow::Result;
use serde::Deserialize;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// Config
#[derive(Debug, Deserialize, Hash)]
pub struct Config {
    /// Place to travel from.
    from: String,
    /// Place to travel to.
    to: String,
    /// Departure date range in "dd/mm/yyyy" format.
    departure_date: (String, String),
    /// Return date range in "dd/mm/yyyy" format.
    return_date: Option<(String, String)>,
    /// API keys.
    #[serde(skip)]
    keys: Keys,
}

impl Config {
    /// Create a new [`Config`].
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
        return_date: Option<(&str, &str)>,
    ) -> Result<Config> {
        Ok(Config {
            from: from.to_owned(),
            to: to.to_owned(),
            departure_date: (departure_date.0.to_owned(), departure_date.1.to_owned()),
            return_date: return_date.map(|x| (x.0.to_owned(), x.1.to_owned())),
            keys: Keys::from_env()?,
        })
    }

    /// Set the API keys from environmental variables.
    pub fn set_keys_from_env(&mut self) -> Result<()> {
        self.keys = Keys::from_env()?;

        Ok(())
    }

    /// Get hash.
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

/// Keys
#[derive(Debug, Hash)]
struct Keys {
    kiwi_search: String,
    kiwi_multicity: String,
    kiwi_nomad: String,
}

impl Default for Keys {
    fn default() -> Self {
        Self::from_env().unwrap_or(Self::empty())
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
