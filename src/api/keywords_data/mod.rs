mod bing;
mod clickstream_data;
mod dataforseo_trends;
mod google_ads;
mod google_trends;

use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

pub use bing::*;
pub use clickstream_data::*;
pub use dataforseo_trends::*;
pub use google_ads::*;
pub use google_trends::*;

/// Entry point for the Keywords Data API domain.
pub struct KeywordsDataApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Keywords Data API (Google Ads, Bing, Google Trends, DataForSEO Trends, Clickstream).
    /// See <https://docs.dataforseo.com/v3/keywords_data/overview/>.
    pub fn keywords_data(&self) -> KeywordsDataApi<'_> {
        KeywordsDataApi { client: self }
    }
}

impl KeywordsDataApi<'_> {
    /// Lists locations supported by a given search engine path, e.g. "google_ads".
    pub async fn locations_se(&self, se: &str) -> DataForSeoApiResponse<KeywordsDataApiLocation> {
        self.client
            .http_get(format!("/v3/keywords_data/{}/locations", se).as_str())
            .await
    }
    /// Lists locations within a country for a given search engine path.
    pub async fn locations_country_se(
        &self,
        se: &str,
        country: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiLocation> {
        self.client
            .http_get(format!("/v3/keywords_data/{}/locations/{}", se, country).as_str())
            .await
    }
    /// Lists completed tasks ready to be collected across all sub-APIs.
    pub async fn task_ready(&self) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client.http_get("/v3/keywords_data/tasks_ready").await
    }
    /// Lists completed tasks ready to be collected for a given search engine path.
    pub async fn task_ready_se(
        &self,
        se: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .http_get(format!("/v3/keywords_data/{}/tasks_ready", se).as_str())
            .await
    }
    /// Lists tasks that were re-run after an error for a given search engine path.
    pub async fn task_fixed_se(
        &self,
        se: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .http_get(format!("/v3/keywords_data/{}/tasks_fixed", se).as_str())
            .await
    }
    /// Lists the Keywords Data tasks created within a time range.
    /// See <https://docs.dataforseo.com/v3/keywords_data/id_list/>.
    pub async fn id_list(
        &self,
        data: Vec<crate::entity::DataForSeoIdListRequest>,
    ) -> crate::DataForSeoApiResponse<crate::entity::DataForSeoIdListItem> {
        self.client
            .http_post("/v3/keywords_data/id_list", &data)
            .await
    }
    /// Lists the Keywords Data tasks that returned an error in the last 7 days.
    /// See <https://docs.dataforseo.com/v3/keywords_data/errors/>.
    pub async fn errors(
        &self,
        data: Vec<crate::entity::DataForSeoErrorsRequest>,
    ) -> crate::DataForSeoApiResponse<crate::entity::DataForSeoErrorItem> {
        self.client
            .http_post("/v3/keywords_data/errors", &data)
            .await
    }
}

/// An entry from a `tasks_ready` or `tasks_fixed` listing.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/search_volume/tasks_ready/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiTaskReadyResult {
    /// Identifier of the completed task, UUID format.
    pub id: Option<String>,
    /// Search engine the task was posted for (e.g. `google_ads`).
    pub se: Option<String>,
    /// Search engine type, where applicable.
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

/// A location supported by a Keywords Data sub-API.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/locations/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiLocation {
    /// Numeric location identifier.
    pub location_code: Option<i32>,
    /// Full location name.
    pub location_name: Option<String>,
    /// Code of the parent location, if any.
    pub location_code_parent: Option<i32>,
    /// ISO country code of the location.
    pub country_iso_code: Option<String>,
    /// Location type, e.g. "Country", "Region" or "City".
    pub location_type: Option<String>,
}
