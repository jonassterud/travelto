#[cfg(test)]
mod tests;

pub mod api;
pub mod kiwi_api;

use anyhow::Result;
use serde::Deserialize;
use std::{
    fs::{self, File},
    path::PathBuf,
    str::FromStr,
    time::SystemTime,
};
use tide::http::mime;

const MAX_MODIFIED_DIFF_SECS: u64 = 60 * 10;

#[async_std::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    tide::log::start();

    let state = api::State::with_default_templates()?;
    let mut app = tide::with_state(state);

    app.at("/").get(index);
    app.at("/search").get(search);
    app.at("index_style.css")
        .serve_file("src/www/index_style.css")?;
    app.at("index_script.js")
        .serve_file("src/www/index_script.js")?;
    app.at("search_style.css")
        .serve_file("src/www/search_style.css")?;

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn index(req: tide::Request<api::State>) -> tide::Result {
    let path = PathBuf::from_str("target/renders/index.html")?;

    let update_contents = move |path: &PathBuf| -> Result<String> {
        // Create directory
        let mut dir_path = path.clone();
        dir_path.pop(); // remove file and keep only directory
        fs::create_dir_all(dir_path)?;

        // Add file and data to template
        let mut output_file = File::create(path)?;
        let data = serde_json::json!({});
        req.state()
            .reg
            .render_to_write("index", &data, &mut output_file)?;

        Ok(fs::read_to_string(path)?)
    };

    let contents = if cfg!(debug_assertions) {
        update_contents(&path)?
    } else {
        fs::read_to_string(&path).or_else(|_| update_contents(&path))?
    };

    Ok(tide::Response::builder(200)
        .content_type(mime::HTML)
        .body(contents)
        .build())
}

async fn search(req: tide::Request<api::State>) -> tide::Result {
    #[derive(Debug, Deserialize)]
    pub struct Intermediary {
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

    impl TryFrom<Intermediary> for api::Config {
        type Error = anyhow::Error;

        fn try_from(val: Intermediary) -> Result<Self> {
            api::Config::new(
                &val.from,
                &val.to,
                (&val.departure_from, &val.departure_to.unwrap_or_default()),
                (
                    &val.return_from.unwrap_or_default(),
                    &val.return_to.unwrap_or_default(),
                ),
                val.adults,
                val.children,
                val.infants,
            )
        }
    }

    let config: api::Config = req.query::<Intermediary>()?.try_into()?;
    let path = PathBuf::from_str(&format!("target/renders/{}.html", config.get_hash()))?;

    let update_contents = move |path: &PathBuf| -> Result<String> {
        let results = api::get_flights(&config).unwrap_or_default();

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
            Err(_) => update_contents(&path)?,
        }
    };

    Ok(tide::Response::builder(200)
        .content_type(mime::HTML)
        .body(contents)
        .build())
}
