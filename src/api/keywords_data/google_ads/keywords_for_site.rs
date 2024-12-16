use crate::{DataForSeoApiResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api::keywords_data::{ KeywordsDataApiTaskReadyResult};
use crate::api::keywords_data::google_ads::KeywordsDataApiGoogle;
use crate::entity::KeywordsDataApiGoogleAdsKeywordsForSiteTask;

/// KeywordsForSite
/// https://docs.dataforseo.com/v3/keywords_data/google_ads/keywords_for_site/task_post/?bash
impl KeywordsDataApiGoogle<'_> {
    pub async fn keywords_for_site_task_post(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/keywords_data/google_ads/keywords_for_site/task_post",
                &data,
            )
            .await
    }
    pub async fn keywords_for_site_tasks_ready(&self) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client.keywords_data().task_ready_se("google/keywords_for_site").await
    }
    pub async fn keywords_for_site_tasks_fixed(&self) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client.keywords_data().task_fixed_se("google/keywords_for_site").await
    }
    pub async fn keywords_for_site_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsKeywordsForSiteTask> {
        self.client
            .http_get(
                ("https://api.dataforseo.com/v3/keywords_data/google_ads/keywords_for_site/task_get/".to_string()
                    + id)
                    .as_str(),
                &{},
            )
            .await
    }
    pub async fn keywords_for_site_live(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsKeywordsForSiteTask> {
        self.client
            .http_post(
                "https://api.dataforseo.com/v3/keywords_data/google_ads/keywords_for_site/live",
                &data,
            )
            .await
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest {
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
impl KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest {
    pub fn new(language_code: String, location_code: i32) -> Self {
        let request = KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest {
            language_code,
            location_code,
            ..KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest::default()
        };
        request
    }
}
