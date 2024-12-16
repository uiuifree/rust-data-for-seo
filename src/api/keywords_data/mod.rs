mod google_ads;

use serde::{Deserialize, Serialize};
use crate::{DataForSeoApiResponse, DataForSeoClient};

pub use google_ads::*;
pub struct KeywordsDataApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// https://docs.dataforseo.com/v3/keywords_data/overview/?bash
    pub fn keywords_data(&self) -> KeywordsDataApi {
        KeywordsDataApi { client: self }
    }
}


impl KeywordsDataApi<'_> {
    pub async fn locations(&self) -> DataForSeoApiResponse<KeywordsDataApiLocation> {
        self.client
            .http_get("https://api.dataforseo.com/v3/keywords_data/locations", &{})
            .await
    }
    pub async fn locations_se(&self, se: &str) -> DataForSeoApiResponse<KeywordsDataApiLocation> {
        self.client
            .http_get(
                format!("https://api.dataforseo.com/v3/keywords_data/{}/locations", se).as_str(),
                &{},
            )
            .await
    }
    pub async fn locations_country(&self, country: &str) -> DataForSeoApiResponse<KeywordsDataApiLocation> {
        self.client
            .http_get(
                format!("https://api.dataforseo.com/v3/keywords_data/locations/{}", country).as_str(),
                &{},
            )
            .await
    }
    pub async fn locations_country_se(
        &self,
        se: &str,
        country: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiLocation> {
        self.client
            .http_get(
                format!(
                    "https://api.dataforseo.com/v3/keywords_data/{}/locations/{}",
                    se, country
                )
                    .as_str(),
                &{},
            )
            .await
    }
    pub async fn task_ready(&self) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .http_get("https://api.dataforseo.com/v3/keywords_data/tasks_ready", &{})
            .await
    }
    pub async fn task_ready_se(&self, se: &str) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .http_get(
                format!("https://api.dataforseo.com/v3/keywords_data/{}/tasks_ready", se).as_str(),
                &{},
            )
            .await
    }
    pub async fn task_fixed(&self) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .http_get("https://api.dataforseo.com/v3/keywords_data/tasks_fixed", &{})
            .await
    }
    pub async fn task_fixed_se(&self, se: &str) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .http_get(
                format!("https://api.dataforseo.com/v3/keywords_data/{}/tasks_fixed", se).as_str(),
                &{},
            )
            .await
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiTaskReadyResult {
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
pub struct KeywordsDataApiLocation {
    pub location_code: Option<i32>,
    pub location_name: Option<String>,
    pub location_code_parent: Option<i32>,
    pub country_iso_code: Option<String>,
    pub location_type: Option<String>,
}
