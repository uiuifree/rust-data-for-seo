use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

mod baidu;
mod bing;
mod google;
mod naver;
mod seznam;
mod yahoo;
mod youtube;

pub use baidu::*;
pub use bing::*;
pub use google::*;
pub use naver::*;
pub use seznam::*;
pub use yahoo::*;
pub use youtube::*;

/// SERP data model.
pub struct SerpApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Returns the SERP API domain entry point.
    /// See <https://docs.dataforseo.com/v3/serp/overview/>.
    pub fn serp(&self) -> SerpApi<'_> {
        SerpApi { client: self }
    }
}

impl SerpApi<'_> {
    /// Lists all locations supported across the SERP engines.
    pub async fn locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client.http_get("/v3/serp/locations").await
    }
    /// Lists locations supported by the given search engine.
    pub async fn locations_se(&self, se: &str) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(format!("/v3/serp/{}/locations", se).as_str())
            .await
    }
    /// Lists supported locations within the given country.
    pub async fn locations_country(&self, country: &str) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(format!("/v3/serp/locations/{}", country).as_str())
            .await
    }
    /// Lists locations within a country for the given search engine.
    pub async fn locations_country_se(
        &self,
        se: &str,
        country: &str,
    ) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(format!("/v3/serp/{}/locations/{}", se, country).as_str())
            .await
    }
    /// Lists completed tasks that are ready to be collected.
    pub async fn task_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.http_get("/v3/serp/tasks_ready").await
    }
    /// Lists completed tasks for the given search engine.
    pub async fn task_ready_se(&self, se: &str) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get(format!("/v3/serp/{}/tasks_ready", se).as_str())
            .await
    }
    /// Lists tasks that were force-fixed by DataForSEO.
    pub async fn task_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.http_get("/v3/serp/tasks_fixed").await
    }
    /// Lists force-fixed tasks for the given search engine.
    pub async fn task_fixed_se(&self, se: &str) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get(format!("/v3/serp/{}/tasks_fixed", se).as_str())
            .await
    }
}

/// A completed task listed by the Tasks Ready endpoint.
/// See <https://docs.dataforseo.com/v3/serp/google/organic/tasks_ready/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiTaskReadyResult {
    /// Identifier of the completed task, UUID format.
    pub id: Option<String>,
    /// Search engine the task was posted for (e.g. `google`).
    pub se: Option<String>,
    /// Search engine type (e.g. `organic`, `maps`).
    pub se_type: Option<String>,
    /// Date and time when the task was posted, UTC.
    pub date_posted: Option<String>,
    /// User-defined tag echoed from the task request.
    pub tag: Option<String>,
    /// URL for collecting the results in the regular format, if available.
    pub endpoint_regular: Option<String>,
    /// URL for collecting the results in the advanced format, if available.
    pub endpoint_advanced: Option<String>,
    /// URL for collecting the results in the HTML format, if available.
    pub endpoint_html: Option<String>,
}

/// Location SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiLocation {
    /// DataForSEO location code the search was run for.
    pub location_code: Option<i32>,
    /// Full location name (e.g. "London,England,United Kingdom").
    pub location_name: Option<String>,
    /// Location code of the parent region, if any.
    pub location_code_parent: Option<i32>,
    /// ISO country code of the location.
    pub country_iso_code: Option<String>,
    /// Type of the location (e.g. City, Region, Country).
    pub location_type: Option<String>,
}
