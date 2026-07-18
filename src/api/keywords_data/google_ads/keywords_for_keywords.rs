use crate::DataForSeoApiResponse;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::api::keywords_data::google_ads::KeywordsDataApiGoogle;
use crate::entity::KeywordsDataApiGoogleAdsKeywordsForKeywordsTask;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Ads Keywords For Keywords.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/keywords_for_keywords/task_post/>.
impl KeywordsDataApiGoogle<'_> {
    /// Posts a Keywords For Keywords task for asynchronous processing.
    pub async fn keywords_for_keywords_task_post(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsKeywordsForKeywordsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "/v3/keywords_data/google_ads/keywords_for_keywords/task_post",
                &data,
            )
            .await
    }
    /// Lists completed Keywords For Keywords tasks ready to be collected.
    pub async fn keywords_for_keywords_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("google/keywords_for_keywords")
            .await
    }
    /// Lists Keywords For Keywords tasks that were re-run after an error.
    pub async fn keywords_for_keywords_tasks_fixed(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_fixed_se("google/keywords_for_keywords")
            .await
    }
    /// Collects the result of a previously posted Keywords For Keywords task.
    pub async fn keywords_for_keywords_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsKeywordsForKeywordsTask> {
        self.client
            .http_get(&format!(
                "/v3/keywords_data/google_ads/keywords_for_keywords/task_get/{id}"
            ))
            .await
    }
    /// Retrieves Keywords For Keywords data synchronously.
    pub async fn keywords_for_keywords_live(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsKeywordsForKeywordsTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsKeywordsForKeywordsTask> {
        self.client
            .http_post(
                "/v3/keywords_data/google_ads/keywords_for_keywords/live",
                &data,
            )
            .await
    }
}

/// Request body for the Google Ads Keywords For Keywords endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForKeywordsTaskPostRequest {
    /// Numeric location identifier.
    pub location_code: i32,
    /// ISO 639-1 language code, e.g. "en".
    pub language_code: String,
    /// Seed keywords to expand (up to 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// Target domain or URL to source additional seed keywords from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Interpretation of `target`, e.g. "site" or "page".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// Full location name; alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// GPS coordinates as "latitude,longitude".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name; alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Include Google search partner data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_partners: Option<bool>,
    /// Start of the historical range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End of the historical range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Include adult keywords in the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_adult_keywords: Option<bool>,
    /// Field to sort the returned keywords by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// URL the API POSTs the result to when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// URL the API pings when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiGoogleAdsKeywordsForKeywordsTaskPostRequest {
    /// Creates a request for the given language and location codes.
    pub fn new(language_code: String, location_code: i32) -> Self {
        KeywordsDataApiGoogleAdsKeywordsForKeywordsTaskPostRequest {
            language_code,
            location_code,
            ..KeywordsDataApiGoogleAdsKeywordsForKeywordsTaskPostRequest::default()
        }
    }
}
