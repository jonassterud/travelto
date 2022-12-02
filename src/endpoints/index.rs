use crate::api;

use anyhow::Result;
use std::{
    fs::{self, File},
    path::PathBuf,
    str::FromStr,
};
use tide::http::mime;

/// Index endpoint.
///
/// # Arguments
///
/// * `req` - a [`tide::Request`] containing the [`api::State`].
pub async fn index(req: tide::Request<api::State>) -> tide::Result {
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
