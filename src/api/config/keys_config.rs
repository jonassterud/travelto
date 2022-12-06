use anyhow::Result;

/// Keys.
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
