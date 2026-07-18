use crate::api::serp::SerpApi;
use crate::entity::{SerpApiNaverOrganicTaskAdvanced, SerpApiNaverOrganicTaskHtml};
use crate::{DataForSeoApiResponse, DataForSeoClient, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl SerpApi<'_> {
    /// Returns the Naver SERP sub-API.
    /// See <https://docs.dataforseo.com/v3/serp/naver/overview/>.
    pub fn naver(&self) -> SerpApiNaver<'_> {
        SerpApiNaver {
            client: self.client,
        }
    }
}

/// Naver SERP data model.
pub struct SerpApiNaver<'a> {
    client: &'a DataForSeoClient,
}

impl SerpApiNaver<'_> {
    /// <https://docs.dataforseo.com/v3/serp/naver/organic/task_post/>
    pub async fn organic_task_post(
        &self,
        data: Vec<SerpApiNaverOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/naver/organic/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/naver/organic/tasks_ready/>
    pub async fn organic_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("naver").await
    }
    /// <https://docs.dataforseo.com/v3/serp/naver/organic/task_get/advanced/>
    pub async fn organic_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiNaverOrganicTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/naver/organic/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/naver/organic/task_get/html/>
    pub async fn organic_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiNaverOrganicTaskHtml> {
        self.client
            .http_get(format!("/v3/serp/naver/organic/task_get/html/{id}").as_str())
            .await
    }
}

/// Request body for Naver organic task_post.
/// Naver results do not vary by location/language, so those parameters are omitted.
/// See <https://docs.dataforseo.com/v3/serp/naver/organic/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiNaverOrganicTaskPostRequest {
    /// Search term the result was returned for.
    pub keyword: String,
    /// URL of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Task execution priority: 1 (normal) or 2 (high).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// Depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Max crawl pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_crawl_pages: Option<i32>,
    /// Device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Os.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Search-engine domain the results were taken from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
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

impl SerpApiNaverOrganicTaskPostRequest {
    /// Creates a request for the given search term.
    pub fn new<T: Into<String>>(keyword: T) -> Self {
        SerpApiNaverOrganicTaskPostRequest {
            keyword: keyword.into(),
            ..SerpApiNaverOrganicTaskPostRequest::default()
        }
    }
}
