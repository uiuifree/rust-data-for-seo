use crate::api::serp::SerpApi;
use crate::entity::{SerpApiYahooOrganicTaskAdvanced, SerpApiYahooOrganicTaskHtml};
use crate::{DataForSeoApiResponse, DataForSeoClient, SerpApiLocation, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl SerpApi<'_> {
    /// Returns the Yahoo SERP sub-API.
    /// See <https://docs.dataforseo.com/v3/serp/yahoo/overview/>.
    pub fn yahoo(&self) -> SerpApiYahoo<'_> {
        SerpApiYahoo {
            client: self.client,
        }
    }
}

/// Yahoo SERP data model.
pub struct SerpApiYahoo<'a> {
    client: &'a DataForSeoClient,
}

impl SerpApiYahoo<'_> {
    /// <https://docs.dataforseo.com/v3/serp/yahoo/locations/>
    pub async fn locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client.serp().locations_se("yahoo").await
    }
    /// <https://docs.dataforseo.com/v3/serp/yahoo/organic/task_post/>
    pub async fn organic_task_post(
        &self,
        data: Vec<SerpApiYahooOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/yahoo/organic/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/yahoo/organic/tasks_ready/>
    pub async fn organic_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("yahoo").await
    }
    /// <https://docs.dataforseo.com/v3/serp/yahoo/organic/task_get/advanced/>
    pub async fn organic_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiYahooOrganicTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/yahoo/organic/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/yahoo/organic/task_get/html/>
    pub async fn organic_task_get_html(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiYahooOrganicTaskHtml> {
        self.client
            .http_get(format!("/v3/serp/yahoo/organic/task_get/html/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/yahoo/organic/live/advanced/>
    pub async fn organic_live_advanced(
        &self,
        data: Vec<SerpApiYahooOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiYahooOrganicTaskAdvanced> {
        self.client
            .http_post("/v3/serp/yahoo/organic/live/advanced", &data)
            .await
    }
}

/// Request body for Yahoo organic task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/yahoo/organic/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiYahooOrganicTaskPostRequest {
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
    /// Device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Os.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Max crawl pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_crawl_pages: Option<i32>,
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

impl SerpApiYahooOrganicTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiYahooOrganicTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiYahooOrganicTaskPostRequest::default()
        }
    }
}
