use crate::entity::SerpApiGoogleEventAdvance;
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiLocation, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Section Job
/// https://docs.dataforseo.com/v3/serp/google/events/overview/?bash
impl SerpApiGoogle<'_> {
    pub async fn events_locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client.serp().locations_se("google/events").await
    }

    pub async fn events_task_post(
        &self,
        data: Vec<SerpApiGoogleEventsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/serp/google/events/task_post",
                &data,
            )
            .await
    }
    pub async fn events_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("google/events").await
    }
    pub async fn events_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_fixed_se("google/events").await
    }

    pub async fn events_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleEventAdvance> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/serp/google/events/task_get/advanced/".to_string()
                    + id)
                    .as_str(),
                &{},
            )
            .await
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleEventsTaskPostRequest {
    pub keyword: String,
    pub priority: Option<i32>,
    pub location_name: Option<String>,
    pub location_code: i32,
    pub location_coordinate: Option<String>,
    pub language_name: Option<String>,
    pub language_code: String,
    pub date_range: Option<String>,
    pub os: Option<String>,
    pub se_domain: Option<String>,
    pub depth: Option<i32>,
    pub max_crawl_pages: Option<i32>,
    pub tag: Option<String>,
    pub postback_url: Option<String>,
    pub postback_data: Option<String>,
    pub pingback_url: Option<String>,
}
impl SerpApiGoogleEventsTaskPostRequest {
    pub fn new(language_code: String, location_code: i32) -> Self {
        let request = SerpApiGoogleEventsTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleEventsTaskPostRequest::default()
        };
        request
    }
}
