use crate::entity::{SerpApiGoogleDatasetInfoAdvanced, SerpApiGoogleDatasetSearchAdvanced};
use crate::{DataForSeoApiResponse, SerpApiGoogle, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Dataset Search & Dataset Info SERP endpoints.
/// <https://docs.dataforseo.com/v3/serp/google/dataset_search/overview/>
impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/dataset_search/task_post/>
    pub async fn dataset_search_task_post(
        &self,
        data: Vec<SerpApiGoogleDatasetSearchTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/dataset_search/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/dataset_search/tasks_ready/>
    pub async fn dataset_search_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_ready_se("google/dataset_search")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/dataset_search/task_get/advanced/>
    pub async fn dataset_search_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleDatasetSearchAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/dataset_search/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/dataset_search/live/advanced/>
    pub async fn dataset_search_live_advanced(
        &self,
        data: Vec<SerpApiGoogleDatasetSearchTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleDatasetSearchAdvanced> {
        self.client
            .http_post("/v3/serp/google/dataset_search/live/advanced", &data)
            .await
    }

    /// <https://docs.dataforseo.com/v3/serp/google/dataset_info/task_post/>
    pub async fn dataset_info_task_post(
        &self,
        data: Vec<SerpApiGoogleDatasetInfoTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/dataset_info/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/dataset_info/tasks_ready/>
    pub async fn dataset_info_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .serp()
            .task_ready_se("google/dataset_info")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/dataset_info/task_get/advanced/>
    pub async fn dataset_info_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleDatasetInfoAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/dataset_info/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/dataset_info/live/advanced/>
    pub async fn dataset_info_live_advanced(
        &self,
        data: Vec<SerpApiGoogleDatasetInfoTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleDatasetInfoAdvanced> {
        self.client
            .http_post("/v3/serp/google/dataset_info/live/advanced", &data)
            .await
    }
}

/// Request body for Google Dataset Search task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/google/dataset_search/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleDatasetSearchTaskPostRequest {
    /// Search term the result was returned for.
    pub keyword: String,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Full language name of the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code the search was run for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// File formats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_formats: Option<Vec<String>>,
    /// Usage rights.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_rights: Option<String>,
    /// `true` if the result is free.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_free: Option<bool>,
    /// Topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
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

impl SerpApiGoogleDatasetSearchTaskPostRequest {
    /// Creates a request for the given dataset search term.
    pub fn new<T: Into<String>>(keyword: T) -> Self {
        SerpApiGoogleDatasetSearchTaskPostRequest {
            keyword: keyword.into(),
            ..SerpApiGoogleDatasetSearchTaskPostRequest::default()
        }
    }
}

/// Request body for Google Dataset Info task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/google/dataset_info/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleDatasetInfoTaskPostRequest {
    /// Google Dataset Search identifier.
    pub dataset_id: String,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Full language name of the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code the search was run for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
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

impl SerpApiGoogleDatasetInfoTaskPostRequest {
    /// Creates a request for the given dataset id.
    pub fn new<T: Into<String>>(dataset_id: T) -> Self {
        SerpApiGoogleDatasetInfoTaskPostRequest {
            dataset_id: dataset_id.into(),
            ..SerpApiGoogleDatasetInfoTaskPostRequest::default()
        }
    }
}
