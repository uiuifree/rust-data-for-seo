use crate::api::serp::SerpApi;
use crate::entity::{SerpApiSeznamOrganicTaskAdvanced, SerpApiSeznamOrganicTaskHtml};
use crate::{DataForSeoApiResponse, DataForSeoClient, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl SerpApi<'_> {
    /// Returns the Seznam SERP sub-API.
    /// See <https://docs.dataforseo.com/v3/serp/seznam/overview/>.
    pub fn seznam(&self) -> SerpApiSeznam<'_> {
        SerpApiSeznam {
            client: self.client,
        }
    }
}

/// Seznam SERP data model.
pub struct SerpApiSeznam<'a> {
    client: &'a DataForSeoClient,
}

impl SerpApiSeznam<'_> {
    /// <https://docs.dataforseo.com/v3/serp/seznam/organic/task_post/>
    pub async fn organic_task_post(
        &self,
        data: Vec<SerpApiSeznamOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/seznam/organic/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/seznam/organic/tasks_ready/>
    pub async fn organic_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("seznam").await
    }
    /// <https://docs.dataforseo.com/v3/serp/seznam/organic/task_get/advanced/>
    pub async fn organic_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiSeznamOrganicTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/seznam/organic/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/seznam/organic/task_get/html/>
    pub async fn organic_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiSeznamOrganicTaskHtml> {
        self.client
            .http_get(format!("/v3/serp/seznam/organic/task_get/html/{id}").as_str())
            .await
    }
}

/// Request body for Seznam organic task_post.
/// See <https://docs.dataforseo.com/v3/serp/seznam/organic/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiSeznamOrganicTaskPostRequest {
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
    /// Calculate rectangles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculate_rectangles: Option<bool>,
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

impl SerpApiSeznamOrganicTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiSeznamOrganicTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiSeznamOrganicTaskPostRequest::default()
        }
    }
}
