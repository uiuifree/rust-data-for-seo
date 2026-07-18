use crate::SerpApiYoutube;
use crate::entity::SerpApiYoutubeOrganicTaskAdvanced;
use crate::{DataForSeoApiResponse, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// YouTube Organic SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/youtube/organic/overview/>
impl SerpApiYoutube<'_> {
    /// <https://docs.dataforseo.com/v3/serp/youtube/organic/task_post/>
    pub async fn organic_task_post(
        &self,
        data: Vec<SerpApiYoutubeOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/youtube/organic/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/youtube/organic/tasks_ready/>
    pub async fn organic_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("youtube/organic").await
    }
    /// <https://docs.dataforseo.com/v3/serp/youtube/organic/task_get/advanced/>
    pub async fn organic_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiYoutubeOrganicTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/youtube/organic/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/youtube/organic/live/advanced/>
    pub async fn organic_live_advanced(
        &self,
        data: Vec<SerpApiYoutubeOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiYoutubeOrganicTaskAdvanced> {
        self.client
            .http_post("/v3/serp/youtube/organic/live/advanced", &data)
            .await
    }
}

/// Request body for YouTube Organic task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/youtube/organic/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiYoutubeOrganicTaskPostRequest {
    /// Search term the result was returned for.
    pub keyword: String,
    /// URL of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Block depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_depth: Option<i32>,
    /// Full location name (e.g. "London,England,United Kingdom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: i32,
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
    /// Search param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_param: Option<String>,
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

impl SerpApiYoutubeOrganicTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiYoutubeOrganicTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiYoutubeOrganicTaskPostRequest::default()
        }
    }
}
