use crate::api::serp::SerpApi;
use crate::entity::{SerpApiBaiduOrganicTaskAdvanced, SerpApiBaiduOrganicTaskHtml};
use crate::{DataForSeoApiResponse, DataForSeoClient, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl SerpApi<'_> {
    /// Returns the Baidu SERP sub-API.
    /// See <https://docs.dataforseo.com/v3/serp/baidu/overview/>.
    pub fn baidu(&self) -> SerpApiBaidu<'_> {
        SerpApiBaidu {
            client: self.client,
        }
    }
}

/// Baidu SERP data model.
pub struct SerpApiBaidu<'a> {
    client: &'a DataForSeoClient,
}

impl SerpApiBaidu<'_> {
    /// <https://docs.dataforseo.com/v3/serp/baidu/organic/task_post/>
    pub async fn organic_task_post(
        &self,
        data: Vec<SerpApiBaiduOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/baidu/organic/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/baidu/organic/tasks_ready/>
    pub async fn organic_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("baidu").await
    }
    /// <https://docs.dataforseo.com/v3/serp/baidu/organic/task_get/advanced/>
    pub async fn organic_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiBaiduOrganicTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/baidu/organic/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/baidu/organic/task_get/html/>
    pub async fn organic_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiBaiduOrganicTaskHtml> {
        self.client
            .http_get(format!("/v3/serp/baidu/organic/task_get/html/{id}").as_str())
            .await
    }
}

/// Request body for Baidu organic task_post.
/// See <https://docs.dataforseo.com/v3/serp/baidu/organic/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiBaiduOrganicTaskPostRequest {
    /// Search term the result was returned for.
    pub keyword: String,
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
    /// URL of the get website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_website_url: Option<bool>,
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

impl SerpApiBaiduOrganicTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiBaiduOrganicTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiBaiduOrganicTaskPostRequest::default()
        }
    }
}
