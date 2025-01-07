use crate::api::keywords_data::google_ads::KeywordsDataApiGoogle;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::entity::KeywordsDataApiGoogleAdsAdTrafficByKeywordsTask;
use crate::DataForSeoApiResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// KeywordsForSite
/// https://docs.dataforseo.com/v3/keywords_data/google_ads/ad_traffic_by_keywords/task_post/?bash
impl KeywordsDataApiGoogle<'_> {
    pub async fn ad_traffic_by_keywords_task_post(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/keywords_data/google_ads/ad_traffic_by_keywords/task_post",
                &data,
            )
            .await
    }
    pub async fn ad_traffic_by_keywords_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("google/ad_traffic_by_keywords")
            .await
    }
    pub async fn ad_traffic_by_keywords_tasks_fixed(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_fixed_se("google/ad_traffic_by_keywords")
            .await
    }
    pub async fn ad_traffic_by_keywords_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsAdTrafficByKeywordsTask> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/keywords_data/google_ads/ad_traffic_by_keywords/task_get/".to_string()
                    + id)
                    .as_str(),
                &{},
            )
            .await
    }
    pub async fn ad_traffic_by_keywords_live(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsAdTrafficByKeywordsTask> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/keywords_data/google_ads/ad_traffic_by_keywords/live",
                &data,
            )
            .await
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest {
    pub keywords: Option<Vec<String>>,
    pub target: Option<String>,
    pub target_type: Option<String>,
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
impl KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest {
    pub fn new(language_code: String, location_code: i32) -> Self {
        let request = KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest {
            language_code,
            location_code,
            ..KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest::default()
        };
        request
    }
}
