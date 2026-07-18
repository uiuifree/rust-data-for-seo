use crate::SerpApiYoutube;
use crate::entity::SerpApiYoutubeVideoInfoTaskAdvanced;
use crate::{DataForSeoApiResponse, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// YouTube Video Info SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/youtube/video_info/overview/>
impl SerpApiYoutube<'_> {
    /// <https://docs.dataforseo.com/v3/serp/youtube/video_info/task_post/>
    pub async fn video_info_task_post(
        &self,
        data: Vec<SerpApiYoutubeVideoInfoTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/youtube/video_info/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/youtube/video_info/tasks_ready/>
    pub async fn video_info_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("youtube/video_info").await
    }
    /// <https://docs.dataforseo.com/v3/serp/youtube/video_info/task_get/advanced/>
    pub async fn video_info_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiYoutubeVideoInfoTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/youtube/video_info/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/youtube/video_info/live/advanced/>
    pub async fn video_info_live_advanced(
        &self,
        data: Vec<SerpApiYoutubeVideoInfoTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiYoutubeVideoInfoTaskAdvanced> {
        self.client
            .http_post("/v3/serp/youtube/video_info/live/advanced", &data)
            .await
    }
}

/// Request body for YouTube Video Info task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/youtube/video_info/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiYoutubeVideoInfoTaskPostRequest {
    /// YouTube video identifier.
    pub video_id: String,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
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

impl SerpApiYoutubeVideoInfoTaskPostRequest {
    /// Creates a request for the given video id with required language and location codes.
    pub fn new<T: Into<String>>(video_id: T, language_code: String, location_code: i32) -> Self {
        SerpApiYoutubeVideoInfoTaskPostRequest {
            video_id: video_id.into(),
            language_code,
            location_code,
            ..SerpApiYoutubeVideoInfoTaskPostRequest::default()
        }
    }
}
