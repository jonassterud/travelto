use crate::api;

use anyhow::Result;
use serde::Deserialize;
use tide::http::mime;

/// Locations endpoint.
///
/// # Arguments
///
/// * `req` - a [`tide::Request`] containing the [`api::State`].
pub async fn locations(req: tide::Request<api::State>) -> tide::Result {
    /// Intermediary struct to catch response data.
    #[derive(Debug, Deserialize)]
    struct Intermediary {
        pub term: String,
    }

    impl TryFrom<Intermediary> for api::LocationConfig {
        type Error = anyhow::Error;

        fn try_from(val: Intermediary) -> Result<Self> {
            Self::new(&val.term)
        }
    }

    let config: api::LocationConfig = req.query::<Intermediary>()?.try_into()?;
    let results = api::get_locations(config).unwrap_or_default(); // should match instead...

    Ok(tide::Response::builder(200)
        .content_type(mime::JSON)
        .body(serde_json::to_string(&results)?)
        .build())
}
