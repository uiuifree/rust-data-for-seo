use crate::entity::SerpApiGoogleEventAdvance;
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiLocation, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Events SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/google/events/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/events/locations/>
    pub async fn events_locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client.serp().locations_se("google/events").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/events/task_post/>
    pub async fn events_task_post(
        &self,
        data: Vec<SerpApiGoogleEventsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/events/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/events/tasks_ready/>
    pub async fn events_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("google/events").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/events/tasks_fixed/>
    pub async fn events_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_fixed_se("google/events").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/events/task_get/advanced/>
    pub async fn events_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleEventAdvance> {
        self.client
            .http_get(format!("/v3/serp/google/events/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/events/live/advanced/>
    pub async fn events_live_advanced(
        &self,
        data: Vec<SerpApiGoogleEventsTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleEventAdvance> {
        self.client
            .http_post("/v3/serp/google/events/live/advanced", &data)
            .await
    }
}

/// Request body for Google Events task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/google/events/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleEventsTaskPostRequest {
    /// Search term the result was returned for.
    pub keyword: String,
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
    /// Date range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range: Option<String>,
    /// Os.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Search-engine domain the results were taken from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
    /// Depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Max crawl pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_crawl_pages: Option<i32>,
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

impl SerpApiGoogleEventsTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleEventsTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleEventsTaskPostRequest::default()
        }
    }
}
