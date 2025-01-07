use crate::api::keywords_data::google_ads::KeywordsDataApiGoogle;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::entity::KeywordsDataApiGoogleAdsSearchVolumeTask;
use crate::DataForSeoApiResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// SearchVolume
/// https://docs.dataforseo.com/v3/keywords_data/google_ads/search_volume/task_post/?bash
impl KeywordsDataApiGoogle<'_> {
    pub async fn search_volume_task_post(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/keywords_data/google_ads/search_volume/task_post",
                &data,
            )
            .await
    }
    pub async fn search_volume_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("google/search_volume")
            .await
    }
    pub async fn search_volume_tasks_fixed(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_fixed_se("google/search_volume")
            .await
    }
    pub async fn search_volume_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsSearchVolumeTask> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/keywords_data/google_ads/search_volume/task_get/"
                    .to_string()
                    + id)
                    .as_str(),
                &{},
            )
            .await
    }
    pub async fn search_volume_live(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsSearchVolumeTask> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/keywords_data/google_ads/search_volume/live",
                &data,
            )
            .await
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest {
    pub keywords: Vec<String>,
    pub location_name: Option<String>,
    pub location_code: i32,
    pub location_coordinate: Option<String>,
    pub language_name: Option<String>,
    pub language_code: String,
    pub search_partners: Option<bool>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub include_adult_keywords: Option<bool>,
    pub sort_by: Option<String>,
    pub postback_url: Option<String>,
    pub pingback_url: Option<String>,
    pub tag: Option<String>,
}
impl KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest {
    pub fn new(language_code: String, location_code: i32) -> Self {
        let request = KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest {
            language_code,
            location_code,
            ..KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest::default()
        };
        request
    }
}
