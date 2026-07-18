use crate::entity::{SerpApiGoogleSearchByImageTaskAdvanced, SerpApiGoogleSearchByImageTaskHtml};
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Search By Image SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/google/search_by_image/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/search_by_image/task_post/>
    pub async fn search_by_image_task_post(
        &self,
        data: Vec<SerpApiGoogleSearchByImageTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/search_by_image/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/search_by_image/tasks_ready/>
    pub async fn search_by_image_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_ready_se("google/search_by_image")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/search_by_image/task_get/advanced/>
    pub async fn search_by_image_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleSearchByImageTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/search_by_image/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/search_by_image/task_get/html/>
    pub async fn search_by_image_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleSearchByImageTaskHtml> {
        self.client
            .http_get(format!("/v3/serp/google/search_by_image/task_get/html/{id}").as_str())
            .await
    }
}

/// Request body for Google Search By Image task_post.
/// See <https://docs.dataforseo.com/v3/serp/google/search_by_image/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleSearchByImageTaskPostRequest {
    /// URL of the image.
    pub image_url: String,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
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
    /// Max crawl pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_crawl_pages: Option<i32>,
    /// Calculate rectangles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculate_rectangles: Option<bool>,
    /// Browser screen width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_width: Option<i32>,
    /// Browser screen height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_height: Option<i32>,
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

impl SerpApiGoogleSearchByImageTaskPostRequest {
    /// Creates a request for the given image URL with required language and location codes.
    pub fn new<T: Into<String>>(image_url: T, language_code: String, location_code: i32) -> Self {
        SerpApiGoogleSearchByImageTaskPostRequest {
            image_url: image_url.into(),
            language_code,
            location_code,
            ..SerpApiGoogleSearchByImageTaskPostRequest::default()
        }
    }
}
