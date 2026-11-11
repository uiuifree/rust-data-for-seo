use crate::entity::SerpApiGoogleAiModeTaskAdvanced;
use crate::{DataForSeoApiResponse, SerpApiGoogle};
use serde::{Deserialize, Serialize};

/// Google AI Mode SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/google/ai_mode/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/ai_mode/live/advanced/>
    pub async fn ai_mode_live_advanced(
        &self,
        data: Vec<SerpApiGoogleAiModeLiveRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleAiModeTaskAdvanced> {
        self.client
            .http_post("/v3/serp/google/ai_mode/live/advanced", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/ai_mode/languages/>
    pub async fn ai_mode_languages(
        &self,
    ) -> crate::DataForSeoApiResponse<crate::entity::SerpApiLanguage> {
        self.client
            .http_get("/v3/serp/google/ai_mode/languages")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/ai_mode/task_post/>
    pub async fn ai_mode_task_post(
        &self,
        data: Vec<SerpApiGoogleAiModeTaskPostRequest>,
    ) -> crate::DataForSeoApiResponse<serde_json::Value> {
        self.client
            .http_post("/v3/serp/google/ai_mode/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/ai_mode/tasks_ready/>
    pub async fn ai_mode_tasks_ready(
        &self,
    ) -> crate::DataForSeoApiResponse<crate::SerpApiTaskReadyResult> {
        self.client
            .http_get("/v3/serp/google/ai_mode/tasks_ready")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/ai_mode/tasks_fixed/>
    pub async fn ai_mode_tasks_fixed(
        &self,
    ) -> crate::DataForSeoApiResponse<crate::SerpApiTaskReadyResult> {
        self.client
            .http_get("/v3/serp/google/ai_mode/tasks_fixed")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/ai_mode/task_get/advanced/>
    pub async fn ai_mode_task_get_advanced(
        &self,
        id: &str,
    ) -> crate::DataForSeoApiResponse<SerpApiGoogleAiModeTaskAdvanced> {
        self.client
            .http_get(&format!("/v3/serp/google/ai_mode/task_get/advanced/{id}"))
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/ai_mode/task_get/html/>
    pub async fn ai_mode_task_get_html(
        &self,
        id: &str,
    ) -> crate::DataForSeoApiResponse<
        crate::entity::SerpApiSearchResult<crate::entity::SerpApiHtmlItem>,
    > {
        self.client
            .http_get(&format!("/v3/serp/google/ai_mode/task_get/html/{id}"))
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/ai_mode/live/html/>
    pub async fn ai_mode_live_html(
        &self,
        data: Vec<SerpApiGoogleAiModeLiveRequest>,
    ) -> crate::DataForSeoApiResponse<
        crate::entity::SerpApiSearchResult<crate::entity::SerpApiHtmlItem>,
    > {
        self.client
            .http_post("/v3/serp/google/ai_mode/live/html", &data)
            .await
    }
}

/// Request body for Google AI Mode live/advanced.
/// See <https://docs.dataforseo.com/v3/serp/google/ai_mode/live/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleAiModeLiveRequest {
    /// Search term the result was returned for.
    pub keyword: String,
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
    /// User-defined identifier echoed back on the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl SerpApiGoogleAiModeLiveRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleAiModeLiveRequest {
            language_code,
            location_code,
            ..SerpApiGoogleAiModeLiveRequest::default()
        }
    }
}

/// Request body for Google AI Mode task_post.
/// See <https://docs.dataforseo.com/v3/serp/google/ai_mode/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleAiModeTaskPostRequest {
    /// Search term.
    pub keyword: String,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Full location name (e.g. "London,England,United Kingdom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// DataForSEO location code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// GPS coordinates ("lat,lng,zoom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Device type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Device operating system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Calculate pixel rankings for SERP elements.
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
    /// User-defined task identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL to send the results to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// Result format sent to `postback_url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_data: Option<String>,
    /// URL to notify when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl SerpApiGoogleAiModeTaskPostRequest {
    /// Creates a request for the given keyword.
    pub fn new<T: Into<String>>(keyword: T) -> Self {
        SerpApiGoogleAiModeTaskPostRequest {
            keyword: keyword.into(),
            ..Default::default()
        }
    }
}
