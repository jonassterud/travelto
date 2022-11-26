mod api;

use anyhow::Result;
use tide::http::mime;
use std::{
    fs::{self, File},
    path::PathBuf,
    str::FromStr,
    time::SystemTime,
};

const MAX_MODIFIED_DIFF_SECS: u64 = 60 * 10;

#[async_std::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    tide::log::start();

    let state = api::State::with_default_templates()?;
    let mut app = tide::with_state(state);

    app.at("/").get(index);
    app.at("/search").get(search);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn index(req: tide::Request<api::State>) -> tide::Result {
    let path = PathBuf::from_str("target/renders/index.html")?;

    let contents = fs::read_to_string(&path).or_else(|_| -> Result<String> {
        // Create directory
        let mut dir_path = path.clone();
        dir_path.pop(); // remove file and keep only directory
        fs::create_dir_all(dir_path)?;

        // Add file and data to template
        let mut output_file = File::create(&path)?;
        let data = serde_json::json!({});
        req.state()
            .reg
            .render_to_write("index", &data, &mut output_file)?;

        Ok(fs::read_to_string(path)?)
    })?;

    Ok(tide::Response::builder(200)
        .content_type(mime::HTML)
        .body(contents)
        .build())
}

async fn search(req: tide::Request<api::State>) -> tide::Result {
    let config: api::Config = req.query()?;
    let path = PathBuf::from_str(&format!("target/renders/{}.html", config.get_hash()))?;

    let update_contents = move |path: &PathBuf| -> Result<String> {
        let results = api::get_flights().unwrap_or_default();

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

    let contents = if let Ok(data) = fs::read_to_string(&path) {
        // Get last modified time
        let modified = fs::metadata(&path)?.modified()?;
        let modified_diff = SystemTime::now().duration_since(modified)?.as_secs();

        // Update contents if time difference exceeds treshold
        if modified_diff > MAX_MODIFIED_DIFF_SECS {
            update_contents(&path)?
        } else {
            data
        }
    } else {
        update_contents(&path)?
    };

    Ok(tide::Response::builder(200)
        .content_type(mime::HTML)
        .body(contents)
        .build())
}
