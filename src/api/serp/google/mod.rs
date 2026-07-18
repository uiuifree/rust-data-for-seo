mod ads_advertisers;
mod ads_search;
mod ai_mode;
mod autocomplete;
mod dataset;
mod event;
mod finance;
mod images;
mod jobs;
mod local_finder;
mod maps;
mod news;
mod search_by_image;

pub use ads_advertisers::*;
pub use ads_search::*;
pub use ai_mode::*;
pub use autocomplete::*;
pub use dataset::*;
pub use event::*;
pub use finance::*;
pub use images::*;
pub use jobs::*;
pub use local_finder::*;
pub use maps::*;
pub use news::*;
pub use search_by_image::*;

use crate::api::serp::SerpApi;
use crate::entity::{
    SerpApiGoogleOrganicTaskAdvanced, SerpApiGoogleOrganicTaskRegular, SerpApiLanguage,
};
use crate::{DataForSeoApiResponse, DataForSeoClient, SerpApiLocation, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl SerpApi<'_> {
    /// Returns the Google SERP sub-API.
    /// See <https://docs.dataforseo.com/v3/serp/google/overview/>.
    pub fn google(&self) -> SerpApiGoogle<'_> {
        SerpApiGoogle {
            client: self.client,
        }
    }
}

/// Google SERP data model.
pub struct SerpApiGoogle<'a> {
    client: &'a DataForSeoClient,
}

impl SerpApiGoogle<'_> {
    /// <https://docs.dataforseo.com/v3/serp/google/locations/>
    pub async fn locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client.http_get("/v3/serp/google/locations").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/locations/>
    pub async fn locations_country(&self, country: &str) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(format!("/v3/serp/google/locations/{country}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/languages/>
    pub async fn languages(&self) -> DataForSeoApiResponse<SerpApiLanguage> {
        self.client.http_get("/v3/serp/google/languages").await
    }

    /// <https://docs.dataforseo.com/v3/serp/google/organic/task_post/>
    pub async fn organic_task_post(
        &self,
        data: Vec<SerpApiGoogleOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/serp/google/organic/task_post", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/organic/tasks_ready/>
    pub async fn organic_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("google").await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/organic/tasks_fixed/>
    pub async fn organic_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get("/v3/serp/google/organic/tasks_fixed")
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/organic/task_get/advanced/>
    pub async fn organic_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleOrganicTaskAdvanced> {
        self.client
            .http_get(format!("/v3/serp/google/organic/task_get/advanced/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/organic/task_get/regular/>
    pub async fn organic_task_get_regular(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleOrganicTaskRegular> {
        self.client
            .http_get(format!("/v3/serp/google/organic/task_get/regular/{id}").as_str())
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/organic/live/advanced/>
    pub async fn organic_live_advanced(
        &self,
        data: Vec<SerpApiGoogleOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleOrganicTaskAdvanced> {
        self.client
            .http_post("/v3/serp/google/organic/live/advanced", &data)
            .await
    }
    /// <https://docs.dataforseo.com/v3/serp/google/organic/live/regular/>
    pub async fn organic_live_regular(
        &self,
        data: Vec<SerpApiGoogleOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<SerpApiGoogleOrganicTaskRegular> {
        self.client
            .http_post("/v3/serp/google/organic/live/regular", &data)
            .await
    }
}

/// Request body for Google organic task_post / live.
/// See <https://docs.dataforseo.com/v3/serp/google/organic/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleOrganicTaskPostRequest {
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
    /// GPS coordinates the search was run for ("lat,lng,zoom").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name of the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code the search was run for.
    pub language_code: String,
    /// Search-engine domain the results were taken from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se_domain: Option<String>,
    /// Device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Os.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Group organic results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_organic_results: Option<bool>,
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
    /// People also ask click depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub people_also_ask_click_depth: Option<i32>,
    /// Load async ai overview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_async_ai_overview: Option<bool>,
    /// Expand ai overview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand_ai_overview: Option<bool>,
    /// Target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
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

impl SerpApiGoogleOrganicTaskPostRequest {
    /// Creates a request with the required language and location codes.
    /// Set `keyword` and any optional fields before sending.
    pub fn new(language_code: String, location_code: i32) -> Self {
        SerpApiGoogleOrganicTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleOrganicTaskPostRequest::default()
        }
    }
}
