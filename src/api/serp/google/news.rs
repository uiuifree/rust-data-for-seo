use crate::entity::{SerpApiGoogleNewsTaskAdvanced, SerpApiGoogleNewsTaskHtml};
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google News SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/google/news/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/news/task_post/>
    pub async fn news_task_post(
        &self,
        data: Vec<SerpApiGoogleNewsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/news/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/news/tasks_ready/>
    pub async fn news_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("google/news").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/news/tasks_fixed/>
    pub async fn news_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_fixed_se("google/news").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/news/task_get/advanced/>
    pub async fn news_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleNewsTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/news/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/news/task_get/html/>
    pub async fn news_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleNewsTaskHtml> {
        self.client
            .http_get(format!("/v3/serp/google/news/task_get/html/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/news/live/advanced/>
    pub async fn news_live_advanced(
        &self,
        data: Vec<SerpApiGoogleNewsTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleNewsTaskAdvanced> {
        self.client
            .http_post("/v3/serp/google/news/live/advanced", &data)
            .await
    }
}

/// Request body for Google News task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/google/news/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleNewsTaskPostRequest {
    /// Search term the result was returned for.
    pub keyword: String,
    /// URL of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
    /// Os.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Calculate rectangles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculate_rectangles: Option<bool>,
    /// Browser screen width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_width: Option<i32>,
    /// Browser screen height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_height: Option<i32>,
    /// Browser screen resolution ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_resolution_ratio: Option<i32>,
    /// Search-engine domain the results were taken from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
    /// Depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Max crawl pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_crawl_pages: Option<i32>,
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

impl SerpApiGoogleNewsTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleNewsTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleNewsTaskPostRequest::default()
        }
    }
}
