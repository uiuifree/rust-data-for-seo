use crate::api::serp::SerpApi;
use crate::entity::{SerpApiBingOrganicTaskAdvanced, SerpApiLanguage};
use crate::{DataForSeoApiResponse, DataForSeoClient, SerpApiLocation, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl SerpApi<'_> {
    /// Returns the Bing SERP sub-API.
    /// See <https://docs.dataforseo.com/v3/serp/bing/overview/>.
    pub fn bing(&self) -> SerpApiBing<'_> {
        SerpApiBing {
            client: self.client,
        }
    }
}

/// Bing SERP data model.
pub struct SerpApiBing<'a> {
    client: &'a DataForSeoClient,
}

impl SerpApiBing<'_> {
    /// <https://docs.dataforseo.com/v3/serp/bing/locations/>
    pub async fn locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client.http_get("/v3/serp/bing/locations").await
    }
    /// <https://docs.dataforseo.com/v3/serp/bing/locations/>
    pub async fn locations_country(&self, country: &str) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(format!("/v3/serp/bing/locations/{country}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/bing/languages/>
    pub async fn languages(&self) -> DataForSeoApiResponse<SerpApiLanguage> {
        self.client.http_get("/v3/serp/bing/languages").await
    }

    /// <https://docs.dataforseo.com/v3/serp/bing/organic/task_post/>
    pub async fn organic_task_post(
        &self,
        data: Vec<SerpApiBingOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/bing/organic/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/bing/organic/tasks_ready/>
    pub async fn organic_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("bing").await
    }
    /// <https://docs.dataforseo.com/v3/serp/bing/organic/tasks_fixed/>
    pub async fn organic_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get("/v3/serp/bing/organic/tasks_fixed")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/bing/organic/task_get/advanced/>
    pub async fn organic_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiBingOrganicTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/bing/organic/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/bing/organic/live/advanced/>
    pub async fn organic_live_advanced(
        &self,
        data: Vec<SerpApiBingOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiBingOrganicTaskAdvanced> {
        self.client
            .http_post("/v3/serp/bing/organic/live/advanced", &data)
            .await
    }
}

/// Request body for Bing organic task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/bing/organic/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiBingOrganicTaskPostRequest {
    /// URL of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
    /// Calculate rectangles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculate_rectangles: Option<bool>,
    /// Browser screen width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_width: Option<i32>,
    /// Browser screen height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_height: Option<i32>,
    /// Browser screen resolution ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_screen_resolution_ratio: Option<i32>,
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

impl SerpApiBingOrganicTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiBingOrganicTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiBingOrganicTaskPostRequest::default()
        }
    }
}
