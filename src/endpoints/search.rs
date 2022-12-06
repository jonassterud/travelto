use crate::api;

use anyhow::Result;
use chrono::NaiveDate;
use serde::Deserialize;
use std::{
    fs::{self, File},
    path::PathBuf,
    str::FromStr,
    time::SystemTime,
};
use tide::http::mime;

const MAX_MODIFIED_DIFF_SECS: u64 = 60 * 10;

/// Search endpoint.
///
/// # Arguments
///
/// * `req` - a [`tide::Request`] containing the [`api::State`].
pub async fn search(req: tide::Request<api::State>) -> tide::Result {
    /// Intermediary struct to catch query data.
    #[derive(Debug, Deserialize)]
    struct Intermediary {
        pub adults: u32,
        pub children: u32,
        pub infants: u32,
        pub from: String,
        pub to: String,
        pub departure_from: String,
        pub departure_to: Option<String>,
        pub return_from: Option<String>,
        pub return_to: Option<String>,
    }

    impl TryFrom<Intermediary> for api::SearchConfig {
        type Error = anyhow::Error;

        fn try_from(val: Intermediary) -> Result<Self> {
            Self::new(
                val.from,
                val.to,
                val.departure_from.parse::<NaiveDate>()?,
                val.departure_to
                    .map(|x| x.parse::<NaiveDate>())
                    .transpose()?,
                val.return_from
                    .map(|x| x.parse::<NaiveDate>())
                    .transpose()?,
                val.return_to.map(|x| x.parse::<NaiveDate>()).transpose()?,
                val.adults,
                val.children,
                val.infants,
            )
        }
    }

    let config: api::SearchConfig = req.query::<Intermediary>()?.try_into()?;
    let path = PathBuf::from_str(&format!("target/renders/{}.html", config.get_hash()))?;

    let update_contents = move |path: &PathBuf| -> Result<String> {
        let results = api::search(config).unwrap_or_default();

        // Create directory
        let mut dir_path = path.clone();
        dir_path.pop(); // remove file and keep only directory
        fs::create_dir_all(dir_path)?;

        // Add file and data to template
        let mut output_file = File::create(path)?;
        let data = serde_json::json!({ "results": results });
        req.state()
            .reg
            .render_to_write("search", &data, &mut output_file)?;

        Ok(fs::read_to_string(path)?)
    };

    let contents = if cfg!(debug_assertions) {
        update_contents(&path)?
    } else {
        match fs::read_to_string(&path) {
            Ok(data) => {
                // Get last modified time
                let modified = fs::metadata(&path)?.modified()?;
                let modified_diff = SystemTime::now().duration_since(modified)?.as_secs();

                // Update contents if time difference exceeds treshold
                if modified_diff > MAX_MODIFIED_DIFF_SECS {
                    update_contents(&path)?
                } else {
                    data
                }
            }
            // Update contents if file doesn't exist
            Err(_) => update_contents(&path)?,
        }
    };

    Ok(tide::Response::builder(200)
        .content_type(mime::HTML)
        .body(contents)
        .build())
}
