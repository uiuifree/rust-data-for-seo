use crate::api::serp::SerpApi;
use crate::entity::{SerpApiBingOrganicTaskAdvanced, SerpApiLanguage};
use crate::{DataForSeoApiResponse, DataForSeoClient, SerpApiLocation, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl SerpApi<'_> {
    /// https://developers.line.biz/ja/reference/messaging-api/#get-bot-info
    pub fn bing(&self) -> SerpApiBing<'_> {
        SerpApiBing {
            client: self.client,
        }
    }
}

pub struct SerpApiBing<'a> {
    client: &'a DataForSeoClient,
}

impl SerpApiBing<'_> {
    /// https://docs.dataforseo.com/v3/serp/bing/locations/
    pub async fn locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get("https://api.dataforseo.com/v3/serp/bing/locations", &{})
            .await
    }
    /// https://docs.dataforseo.com/v3/serp/bing/locations/
    pub async fn locations_country(&self, country: &str) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/serp/bing/locations/".to_string() + country)
                    .as_str(),
                &{},
            )
            .await
    }
    /// https://docs.dataforseo.com/v3/serp/bing/languages/
    pub async fn languages(&self) -> DataForSeoApiResponse<SerpApiLanguage> {
        self.client
            .http_get("https://api.dataforseo.com/v3/serp/bing/languages", &{})
            .await
    }

    /// https://docs.dataforseo.com/v3/serp/bing/organic/task_post/?bash
    pub async fn organic_task_post(
        &self,
        data: Vec<SerpApiBingOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/serp/bing/organic/task_post",
                &data,
            )
            .await
    }
    /// https://docs.dataforseo.com/v3/serp/bing/organic/task_post/?bash
    pub async fn organic_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("bing").await
    }
    /// https://docs.dataforseo.com/v3/serp/bing/organic/tasks_fixed/?bash
    pub async fn organic_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get(
                "https://api.dataforseo.com/v3/serp/bing/organic/tasks_fixed",
                &{},
            )
            .await
    }
    pub async fn organic_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiBingOrganicTaskAdvanced> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/serp/bing/organic/task_get/advanced/".to_string()
                    + id)
                    .as_str(),
                &{},
            )
            .await
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]

pub struct SerpApiBingOrganicTaskPostRequest {
    pub url: Option<String>,
    pub keyword: String,
    pub priority: Option<i32>,
    pub location_name: Option<String>,
    pub location_code: i32,
    pub location_coordinate: Option<String>,
    pub language_name: Option<String>,
    pub language_code: String,
    pub device: Option<String>,
    pub os: Option<String>,
    pub depth: Option<i32>,
    pub max_crawl_pages: Option<i32>,
    pub calculate_rectangles: Option<bool>,
    pub browser_screen_width: Option<i32>,
    pub browser_screen_height: Option<i32>,
    pub browser_screen_resolution_ratio: Option<i32>,
    pub search_param: Option<String>,
    pub tag: Option<String>,
    pub postback_url: Option<String>,
    pub postback_data: Option<String>,
    pub pingback_url: Option<String>,
}

impl SerpApiBingOrganicTaskPostRequest {
    pub fn new(language_code: String, location_code: i32) -> Self {
        let request = SerpApiBingOrganicTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiBingOrganicTaskPostRequest::default()
        };
        request
    }
}
