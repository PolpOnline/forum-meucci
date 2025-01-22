use reqwest::ClientBuilder;

use crate::app::App;

impl App {
    /// Used for fetching the data for OpenID Connect. Does not follow redirects
    /// as that might introduce SSRF vulnerabilities.
    pub(crate) fn get_unredirectable_async_client() -> color_eyre::Result<reqwest::Client> {
        let async_http_client = ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        Ok(async_http_client)
    }
}
