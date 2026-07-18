use crate::entity::{SerpApiGoogleMapsTaskAdvanced, SerpApiGoogleMapsTaskHtml};
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Maps SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/google/maps/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/maps/task_post/>
    pub async fn maps_task_post(
        &self,
        data: Vec<SerpApiGoogleMapsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/maps/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/maps/tasks_ready/>
    pub async fn maps_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("google/maps").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/maps/task_get/advanced/>
    pub async fn maps_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleMapsTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/maps/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/maps/task_get/html/>
    pub async fn maps_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleMapsTaskHtml> {
        self.client
            .http_get(format!("/v3/serp/google/maps/task_get/html/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/maps/live/advanced/>
    pub async fn maps_live_advanced(
        &self,
        data: Vec<SerpApiGoogleMapsTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleMapsTaskAdvanced> {
        self.client
            .http_post("/v3/serp/google/maps/live/advanced", &data)
            .await
    }
}

/// Request body for Google Maps task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/google/maps/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleMapsTaskPostRequest {
    /// Search term the result was returned for.
    pub keyword: String,
    /// URL of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Max crawl pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_crawl_pages: Option<i32>,
    /// Full location name (e.g. "London,England,United Kingdom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: i32,
    /// GPS coordinates the search was run for ("lat,lng,zoom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name of the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code the search was run for.
    pub language_code: String,
    /// Search-engine domain the results were taken from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
    /// Device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Os.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Search places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_places: Option<bool>,
    /// Search this area.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_this_area: Option<bool>,
    /// User-defined identifier echoed back on the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL of the postback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Postback data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL of the pingback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl SerpApiGoogleMapsTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleMapsTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleMapsTaskPostRequest::default()
        }
    }
}
