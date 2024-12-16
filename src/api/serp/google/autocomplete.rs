use crate::entity::SerpApiGoogleAutocompleteAdvanced;
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Section Autocomplete
impl SerpApiGoogle<'_> {
    pub async fn autocomplete_task_post(
        &self,
        data: Vec<SerpApiGoogleAutoCompleteTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/serp/google/autocomplete/task_post",
                &data,
            )
            .await
    }
    pub async fn autocomplete_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_ready_se("google/autocomplete")
            .await
    }
    pub async fn autocomplete_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_fixed_se("google/autocomplete")
            .await
    }
    pub async fn autocomplete_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleAutocompleteAdvanced> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/serp/google/autocomplete/task_get/advanced/"
                    .to_string()
                    + id)
                    .as_str(),
                &{},
            )
            .await
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleAutoCompleteTaskPostRequest {
    pub keyword: String,
    pub priority: Option<i32>,
    pub location_name: Option<String>,
    pub location_code: i32,
    // pub location_radius: Option<String>,
    pub language_name: Option<String>,
    pub language_code: String,
    pub cursor_pointer: Option<String>,
    pub client: Option<String>,
    pub tag: Option<String>,
    pub postback_url: Option<String>,
    pub postback_data: Option<String>,
    pub pingback_url: Option<String>,
}
impl SerpApiGoogleAutoCompleteTaskPostRequest {
    pub fn new(language_code: String, location_code: i32) -> Self {
        let request = SerpApiGoogleAutoCompleteTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleAutoCompleteTaskPostRequest::default()
        };
        request
    }
}
