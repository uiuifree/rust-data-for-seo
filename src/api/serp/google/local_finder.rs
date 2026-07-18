use crate::entity::{SerpApiGoogleLocalFinderTaskAdvanced, SerpApiGoogleLocalFinderTaskHtml};
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Local Finder SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/google/local_finder/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/local_finder/task_post/>
    pub async fn local_finder_task_post(
        &self,
        data: Vec<SerpApiGoogleLocalFinderTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/local_finder/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/local_finder/tasks_ready/>
    pub async fn local_finder_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_ready_se("google/local_finder")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/local_finder/task_get/advanced/>
    pub async fn local_finder_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleLocalFinderTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/local_finder/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/local_finder/task_get/html/>
    pub async fn local_finder_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleLocalFinderTaskHtml> {
        self.client
            .http_get(format!("/v3/serp/google/local_finder/task_get/html/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/local_finder/live/advanced/>
    pub async fn local_finder_live_advanced(
        &self,
        data: Vec<SerpApiGoogleLocalFinderTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleLocalFinderTaskAdvanced> {
        self.client
            .http_post("/v3/serp/google/local_finder/live/advanced", &data)
            .await
    }
}

/// Request body for Google Local Finder task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/google/local_finder/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleLocalFinderTaskPostRequest {
    /// Search term the result was returned for.
    pub keyword: String,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
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
    /// Device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Os.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Min rating.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_rating: Option<i32>,
    /// Time filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_filter: Option<String>,
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

impl SerpApiGoogleLocalFinderTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleLocalFinderTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleLocalFinderTaskPostRequest::default()
        }
    }
}
