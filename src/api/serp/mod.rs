use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};

mod bing;
mod google;

pub use bing::*;
pub use google::*;

pub struct SerpApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// https://developers.line.biz/ja/reference/messaging-api/#get-bot-info
    pub fn serp(&self) -> SerpApi {
        SerpApi { client: self }
    }
}

impl SerpApi<'_> {
    pub async fn locations(&self) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get("https://api.dataforseo.com/v3/serp/locations", &{})
            .await
    }
    pub async fn locations_se(&self, se: &str) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(
                format!("https://api.dataforseo.com/v3/serp/{}/locations", se).as_str(),
                &{},
            )
            .await
    }
    pub async fn locations_country(&self, country: &str) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(
                format!("https://api.dataforseo.com/v3/serp/locations/{}", country).as_str(),
                &{},
            )
            .await
    }
    pub async fn locations_country_se(
        &self,
        se: &str,
        country: &str,
    ) -> DataForSeoApiResponse<SerpApiLocation> {
        self.client
            .http_get(
                format!(
                    "https://api.dataforseo.com/v3/serp/{}/locations/{}",
                    se, country
                )
                .as_str(),
                &{},
            )
            .await
    }
    pub async fn task_ready(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get("https://api.dataforseo.com/v3/serp/tasks_ready", &{})
            .await
    }
    pub async fn task_ready_se(&self, se: &str) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get(
                format!("https://api.dataforseo.com/v3/serp/{}/tasks_ready", se).as_str(),
                &{},
            )
            .await
    }
    pub async fn task_fixed(&self) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get("https://api.dataforseo.com/v3/serp/tasks_fixed", &{})
            .await
    }
    pub async fn task_fixed_se(&self, se: &str) -> DataForSeoApiResponse<SerpApiTaskReadyResult> {
        self.client
            .http_get(
                format!("https://api.dataforseo.com/v3/serp/{}/tasks_fixed", se).as_str(),
                &{},
            )
            .await
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiTaskReadyResult {
    id: Option<String>,
    se: Option<String>,
    se_type: Option<String>,
    date_posted: Option<String>,
    tag: Option<String>,
    endpoint_regular: Option<String>,
    endpoint_advanced: Option<String>,
    endpoint_html: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiLocation {
    pub location_code: Option<i32>,
    pub location_name: Option<String>,
    pub location_code_parent: Option<i32>,
    pub country_iso_code: Option<String>,
    pub location_type: Option<String>,
}
