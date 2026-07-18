use crate::entity::SerpApiGoogleAutocompleteAdvanced;
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Autocomplete SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/google/autocomplete/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/autocomplete/task_post/>
    pub async fn autocomplete_task_post(
        &self,
        data: Vec<SerpApiGoogleAutoCompleteTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/autocomplete/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/autocomplete/tasks_ready/>
    pub async fn autocomplete_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_ready_se("google/autocomplete")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/autocomplete/tasks_fixed/>
    pub async fn autocomplete_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_fixed_se("google/autocomplete")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/autocomplete/task_get/advanced/>
    pub async fn autocomplete_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleAutocompleteAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/autocomplete/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/autocomplete/live/advanced/>
    pub async fn autocomplete_live_advanced(
        &self,
        data: Vec<SerpApiGoogleAutoCompleteTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleAutocompleteAdvanced> {
        self.client
            .http_post("/v3/serp/google/autocomplete/live/advanced", &data)
            .await
    }
}

/// Request body for Google Autocomplete task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/google/autocomplete/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleAutoCompleteTaskPostRequest {
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
    /// Full language name of the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code the search was run for.
    pub language_code: String,
    /// Cursor pointer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_pointer: Option<i32>,
    /// Client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
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

impl SerpApiGoogleAutoCompleteTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleAutoCompleteTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleAutoCompleteTaskPostRequest::default()
        }
    }
}
