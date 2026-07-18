use crate::DataForSeoApiResponse;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::api::keywords_data::google_ads::KeywordsDataApiGoogle;
use crate::entity::KeywordsDataApiGoogleAdsKeywordsForSiteTask;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Ads Keywords For Site.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/keywords_for_site/task_post/>.
impl KeywordsDataApiGoogle<'_> {
    /// Posts a Keywords For Site task for asynchronous processing.
    pub async fn keywords_for_site_task_post(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "/v3/keywords_data/google_ads/keywords_for_site/task_post",
                &data,
            )
            .await
    }
    /// Lists completed Keywords For Site tasks ready to be collected.
    pub async fn keywords_for_site_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("google/keywords_for_site")
            .await
    }
    /// Lists Keywords For Site tasks that were re-run after an error.
    pub async fn keywords_for_site_tasks_fixed(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_fixed_se("google/keywords_for_site")
            .await
    }
    /// Collects the result of a previously posted Keywords For Site task.
    pub async fn keywords_for_site_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsKeywordsForSiteTask> {
        self.client
            .http_get(&format!(
                "/v3/keywords_data/google_ads/keywords_for_site/task_get/{id}"
            ))
            .await
    }
    /// Retrieves Keywords For Site data synchronously.
    pub async fn keywords_for_site_live(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsKeywordsForSiteTask> {
        self.client
            .http_post("/v3/keywords_data/google_ads/keywords_for_site/live", &data)
            .await
    }
}

/// Request body for the Google Ads Keywords For Site endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest {
    /// Numeric location identifier.
    pub location_code: i32,
    /// ISO 639-1 language code, e.g. "en".
    pub language_code: String,
    /// Target domain or URL to source keywords from.
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

impl KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest {
    /// Creates a request for the given language and location codes.
    pub fn new(language_code: String, location_code: i32) -> Self {
        KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest {
            language_code,
            location_code,
            ..KeywordsDataApiGoogleAdsKeywordsForSiteTaskPostRequest::default()
        }
    }
}
