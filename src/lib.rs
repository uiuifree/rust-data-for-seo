//! Rust client for the [DataForSEO API v3](https://docs.dataforseo.com/v3/).
//!
//! # Quick start
//!
//! ```no_run
//! use data_for_seo::DataForSeoClient;
//!
//! # async fn example() -> Result<(), data_for_seo::error::DataForSeoError> {
//! let client = DataForSeoClient::new("login", "password");
//! let response = client.backlinks().summary(vec![
//!     data_for_seo::BacklinksApiSummaryPost::new("https://example.com/"),
//! ]).await?;
//! for result in response.results() {
//!     println!("{:?}", result);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! The client is organized by API domain, mirroring the structure of the
//! DataForSEO documentation:
//!
//! - [`DataForSeoClient::serp`] — SERP API (Google, Bing, YouTube, ...)
//! - [`DataForSeoClient::keywords_data`] — Keywords Data API
//! - [`DataForSeoClient::dataforseo_labs`] — DataForSEO Labs API
//! - [`DataForSeoClient::backlinks`] — Backlinks API
//! - [`DataForSeoClient::on_page`] — OnPage API
//! - [`DataForSeoClient::domain_analytics`] — Domain Analytics API
//! - [`DataForSeoClient::content_analysis`] — Content Analysis API
//! - [`DataForSeoClient::content_generation`] — Content Generation API
//! - [`DataForSeoClient::merchant`] — Merchant API
//! - [`DataForSeoClient::app_data`] — App Data API
//! - [`DataForSeoClient::business_data`] — Business Data API
//! - [`DataForSeoClient::ai_optimization`] — AI Optimization API
//! - [`DataForSeoClient::appendix`] — Appendix (account data)
//!
//! # Response handling
//!
//! Every call returns [`DataForSeoApiResponse`], whose success value wraps the
//! standard DataForSEO envelope. Use [`DataForSeoApiResponseData::results`],
//! [`DataForSeoApiResponseData::tasks`], or
//! [`DataForSeoApiResponseData::first_result`] to reach the payload without
//! unwrapping nested `Option`s.
//!
//! # Sandbox
//!
//! [`DataForSeoClient::sandbox`] switches the client to the free
//! [sandbox environment](https://docs.dataforseo.com/v3/appendix/sandbox/),
//! which returns mock data for most endpoints.

#![warn(missing_docs)]

mod api;
pub mod entity;
pub mod error;
pub use api::*;
pub use entity::*;

use crate::error::{
    DataForSeoApiError, DataForSeoError, DataForSeoHttpError, DataForSeoSystemError,
};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde_json::Value;

/// Production API endpoint.
pub const DEFAULT_BASE_URL: &str = "https://api.dataforseo.com";
/// Sandbox endpoint — free of charge, returns mock data.
/// See <https://docs.dataforseo.com/v3/appendix/sandbox/>.
pub const SANDBOX_BASE_URL: &str = "https://sandbox.dataforseo.com";

/// Entry point for all DataForSEO API calls.
///
/// Holds a shared [`reqwest::Client`] (connection pool) and the Basic-auth
/// token. Cloning is cheap and clones share the connection pool.
#[derive(Debug, Clone)]
pub struct DataForSeoClient {
    http: reqwest::Client,
    base_url: String,
    token: String,
}

impl DataForSeoClient {
    /// Creates a client authenticated with the given API login and password.
    pub fn new<L: Into<String>, P: Into<String>>(login: L, password: P) -> DataForSeoClient {
        DataForSeoClient {
            http: reqwest::Client::new(),
            base_url: DEFAULT_BASE_URL.to_string(),
            token: BASE64_STANDARD.encode(format!("{}:{}", login.into(), password.into())),
        }
    }

    /// Creates a client from the `DATAFORSEO_LOGIN` and `DATAFORSEO_PASSWORD`
    /// environment variables.
    ///
    /// Returns an error naming the missing variable when either is not set.
    pub fn from_env() -> Result<DataForSeoClient, DataForSeoError> {
        let login = std::env::var("DATAFORSEO_LOGIN")
            .map_err(|_| DataForSeoSystemError::new("DATAFORSEO_LOGIN is not set"))?;
        let password = std::env::var("DATAFORSEO_PASSWORD")
            .map_err(|_| DataForSeoSystemError::new("DATAFORSEO_PASSWORD is not set"))?;
        Ok(DataForSeoClient::new(login, password))
    }

    /// Overrides the base URL (e.g. [`SANDBOX_BASE_URL`]).
    pub fn with_base_url<T: Into<String>>(mut self, base_url: T) -> Self {
        self.base_url = base_url.into().trim_end_matches('/').to_string();
        self
    }

    /// Switches the client to the free sandbox environment.
    pub fn sandbox(self) -> Self {
        self.with_base_url(SANDBOX_BASE_URL)
    }

    /// Replaces the underlying HTTP client (to customize timeouts, proxies, ...).
    pub fn with_http_client(mut self, http: reqwest::Client) -> Self {
        self.http = http;
        self
    }
}

/// Result alias returned by every API method.
pub type DataForSeoApiResponse<R> = Result<entity::DataForSeoApiResponseData<R>, DataForSeoError>;

impl DataForSeoClient {
    fn endpoint(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    async fn read_response<R>(response: reqwest::Response) -> DataForSeoApiResponse<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        let status = response.status().as_u16();
        let body = match response.text().await {
            Ok(v) => v,
            Err(e) => return Err(DataForSeoHttpError::new(status, e.to_string()).into()),
        };

        let value = match serde_json::from_str::<Value>(&body) {
            Ok(v) => v,
            Err(_) => return Err(DataForSeoHttpError::new(status, body).into()),
        };

        if status >= 400 {
            return Err(DataForSeoApiError::from_envelope(status, &value, body).into());
        }

        match serde_json::from_value::<entity::DataForSeoApiResponseData<R>>(value.clone()) {
            Ok(v) => Ok(v),
            Err(e) => Err(DataForSeoSystemError::new(format!(
                "failed to deserialize response: {e} body={value}"
            ))
            .into()),
        }
    }

    pub(crate) async fn http_get<R>(&self, path: &str) -> DataForSeoApiResponse<R>
    where
        R: for<'de> serde::Deserialize<'de>,
    {
        let request = self
            .http
            .get(self.endpoint(path))
            .header("Authorization", format!("Basic {}", self.token));
        match request.send().await {
            Ok(response) => Self::read_response(response).await,
            Err(e) => Err(DataForSeoSystemError::new(format!("request failed: {e}")).into()),
        }
    }

    pub(crate) async fn http_post<P, R>(&self, path: &str, body: &P) -> DataForSeoApiResponse<R>
    where
        P: serde::Serialize,
        R: for<'de> serde::Deserialize<'de>,
    {
        let request = self
            .http
            .post(self.endpoint(path))
            .header("Authorization", format!("Basic {}", self.token))
            .json(body);
        match request.send().await {
            Ok(response) => Self::read_response(response).await,
            Err(e) => Err(DataForSeoSystemError::new(format!("request failed: {e}")).into()),
        }
    }
}
