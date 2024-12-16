use crate::entity::{SerpApiGoogleJobAdvanced, SerpApiGoogleJobHtml};
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiLocation, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Section Job
/// https://docs.dataforseo.com/v3/serp/google/jobs/overview/?bash
impl SerpApiGoogle<'_> {
    pub async fn jobs_locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client.serp().locations_se("google/jobs").await
    }

    pub async fn jobs_task_post(
        &self,
        data: Vec<SerpApiGoogleJobsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/serp/google/jobs/task_post",
                &data,
            )
            .await
    }
    pub async fn jobs_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("google/jobs").await
    }
    pub async fn jobs_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_fixed_se("google/jobs").await
    }

    pub async fn jobs_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleJobAdvanced> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/serp/google/jobs/task_get/advanced/".to_string()
                    + id)
                    .as_str(),
                &{},
            )
            .await
    }
    pub async fn jobs_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleJobHtml> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/serp/google/jobs/task_get/html/".to_string() + id)
                    .as_str(),
                &{},
            )
            .await
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleJobsTaskPostRequest {
    pub keyword: String,
    pub priority: Option<i32>,
    pub location_name: Option<String>,
    pub location_code: i32,
    pub location_radius: Option<String>,
    pub language_name: Option<String>,
    pub language_code: String,
    pub depth: Option<i32>,
    pub employment_type: Option<Vec<String>>,
    pub date_posted: Option<String>,
    pub tag: Option<String>,
    pub postback_url: Option<String>,
    pub postback_data: Option<String>,
    pub pingback_url: Option<String>,
}
impl SerpApiGoogleJobsTaskPostRequest {
    pub fn new(language_code: String, location_code: i32) -> Self {
        let request = SerpApiGoogleJobsTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleJobsTaskPostRequest::default()
        };
        request
    }
}
