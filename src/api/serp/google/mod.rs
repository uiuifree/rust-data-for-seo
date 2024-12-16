mod autocomplete;
mod event;
mod images;
mod jobs;
mod news;

pub use autocomplete::*;
pub use event::*;
pub use images::*;
pub use jobs::*;
pub use news::*;

use crate::api::serp::SerpApi;
use crate::entity::{SerpApiGoogleLanguage, SerpApiGoogleOrganicTaskAdvanced};
use crate::{DataForSeoApiResponse, DataForSeoClient, SerpApiLocation, SerpApiTaskReadyResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

impl SerpApi<'_> {
    /// https://developers.line.biz/ja/reference/messaging-api/#get-bot-info
    pub fn google(&self) -> SerpApiGoogle<'_> {
        SerpApiGoogle {
            client: self.client,
        }
    }
}

pub struct SerpApiGoogle<'a> {
    client: &'a DataForSeoClient,
}

impl SerpApiGoogle<'_> {
    /// https://docs.dataforseo.com/v3/serp/google/locations/
    pub async fn locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get("https://api.dataforseo.com/v3/serp/google/locations", &{})
            .await
    }
    /// https://docs.dataforseo.com/v3/serp/google/locations/
    pub async fn locations_country(&self, country: &str) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/serp/google/locations/".to_string() + country)
                    .as_str(),
                &{},
            )
            .await
    }
    /// https://docs.dataforseo.com/v3/serp/google/languages/
    pub async fn languages(&self) -> DataForSeoApiResponse<SerpApiGoogleLanguage> {
        self.client
            .http_get("https://api.dataforseo.com/v3/serp/google/languages", &{})
            .await
    }

    /// https://docs.dataforseo.com/v3/serp/google/organic/task_post/?bash
    pub async fn organic_task_post(
        &self,
        data: Vec<SerpApiGoogleOrganicTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/serp/google/organic/task_post",
                &data,
            )
            .await
    }
    /// https://docs.dataforseo.com/v3/serp/google/organic/task_post/?bash
    pub async fn organic_tasks_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client.serp().task_ready_se("google").await
    }
    /// https://docs.dataforseo.com/v3/serp/google/organic/tasks_fixed/?bash
    pub async fn organic_tasks_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get(
                "https://api.dataforseo.com/v3/serp/google/organic/tasks_fixed",
                &{},
            )
            .await
    }
    pub async fn organic_task_get_advanced(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<SerpApiGoogleOrganicTaskAdvanced> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/serp/google/organic/task_get/advanced/"
                    .to_string()
                    + id)
                    .as_str(),
                &{},
            )
            .await
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]

pub struct SerpApiGoogleOrganicTaskPostRequest {
    pub keyword: String,
    pub url: Option<String>,
    pub priority: Option<i32>,
    pub depth: Option<i32>,
    pub max_crawl_pages: Option<i32>,
    pub location_name: Option<String>,
    pub location_code: i32,
    pub location_coordinate: Option<String>,
    pub language_name: Option<String>,
    pub language_code: String,
    pub se_domain: Option<String>,
    pub device: Option<String>,
    pub os: Option<String>,
    pub group_organic_results: Option<bool>,
    pub calculate_rectangles: Option<bool>,
    pub browser_screen_width: Option<i32>,
    pub browser_screen_height: Option<i32>,
    pub browser_screen_resolution_ratio: Option<i32>,
    pub people_also_ask_click_depth: Option<i32>,
    pub load_async_ai_overview: Option<bool>,
    pub expand_ai_overview: Option<bool>,
    pub search_param: Option<String>,
    pub tag: Option<String>,
    pub postback_url: Option<String>,
    pub postback_data: Option<String>,
    pub pingback_url: Option<String>,
}

impl SerpApiGoogleOrganicTaskPostRequest {
    pub fn new(language_code: String, location_code: i32) -> Self {
        let request = SerpApiGoogleOrganicTaskPostRequest {
            language_code,
            location_code,
            ..SerpApiGoogleOrganicTaskPostRequest::default()
        };
        request
    }
}
