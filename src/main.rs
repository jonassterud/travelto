#[cfg(test)]
mod tests;

pub mod api;
pub mod endpoints;
pub mod kiwi_api;

use anyhow::Result;

#[async_std::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    tide::log::start();

    let state = api::State::with_default_templates()?;
    let mut app = tide::with_state(state);

    app.at("/").get(endpoints::index);
    app.at("/locations").get(endpoints::locations);
    app.at("/search").get(endpoints::search);

    app.at("index_style.css")
        .serve_file("src/www/index_style.css")?;
    app.at("index_script.js")
        .serve_file("src/www/index_script.js")?;
    app.at("search_style.css")
        .serve_file("src/www/search_style.css")?;

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
