use crate::entity::{SerpApiGoogleJobAdvanced, SerpApiGoogleJobHtml};
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiLocation, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Jobs SERP endpoint.
/// <https://docs.dataforseo.com/v3/serp/google/jobs/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/jobs/locations/>
    pub async fn jobs_locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client.serp().locations_se("google/jobs").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/jobs/task_post/>
    pub async fn jobs_task_post(
        &self,
        data: Vec<SerpApiGoogleJobsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/jobs/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/jobs/tasks_ready/>
    pub async fn jobs_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("google/jobs").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/jobs/tasks_fixed/>
    pub async fn jobs_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_fixed_se("google/jobs").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/jobs/task_get/advanced/>
    pub async fn jobs_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleJobAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/jobs/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/jobs/task_get/html/>
    pub async fn jobs_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleJobHtml> {
        self.client
            .http_get(format!("/v3/serp/google/jobs/task_get/html/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/jobs/live/advanced/>
    pub async fn jobs_live_advanced(
        &self,
        data: Vec<SerpApiGoogleJobsTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleJobAdvanced> {
        self.client
            .http_post("/v3/serp/google/jobs/live/advanced", &data)
            .await
    }
}

/// Request body for Google Jobs task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/google/jobs/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleJobsTaskPostRequest {
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
    /// Location radius.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_radius: Option<String>,
    /// Full language name of the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code the search was run for.
    pub language_code: String,
    /// Depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Employment type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<Vec<String>>,
    /// Date posted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_posted: Option<String>,
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

impl SerpApiGoogleJobsTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleJobsTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleJobsTaskPostRequest::default()
        }
    }
}
