use crate::DataForSeoApiResponse;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::api::keywords_data::google_ads::KeywordsDataApiGoogle;
use crate::entity::KeywordsDataApiGoogleAdsAdTrafficByKeywordsTask;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Ads Ad Traffic By Keywords.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/ad_traffic_by_keywords/task_post/>.
impl KeywordsDataApiGoogle<'_> {
    /// Posts an Ad Traffic By Keywords task for asynchronous processing.
    pub async fn ad_traffic_by_keywords_task_post(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "/v3/keywords_data/google_ads/ad_traffic_by_keywords/task_post",
                &data,
            )
            .await
    }
    /// Lists completed Ad Traffic By Keywords tasks ready to be collected.
    pub async fn ad_traffic_by_keywords_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("google/ad_traffic_by_keywords")
            .await
    }
    /// Lists Ad Traffic By Keywords tasks that were re-run after an error.
    pub async fn ad_traffic_by_keywords_tasks_fixed(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_fixed_se("google/ad_traffic_by_keywords")
            .await
    }
    /// Collects the result of a previously posted Ad Traffic By Keywords task.
    pub async fn ad_traffic_by_keywords_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsAdTrafficByKeywordsTask> {
        self.client
            .http_get(&format!(
                "/v3/keywords_data/google_ads/ad_traffic_by_keywords/task_get/{id}"
            ))
            .await
    }
    /// Retrieves Ad Traffic By Keywords data synchronously.
    pub async fn ad_traffic_by_keywords_live(
        &self,
        data: Vec<KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleAdsAdTrafficByKeywordsTask> {
        self.client
            .http_post(
                "/v3/keywords_data/google_ads/ad_traffic_by_keywords/live",
                &data,
            )
            .await
    }
}

/// Request body for the Google Ads Ad Traffic By Keywords endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest {
    /// Numeric location identifier.
    pub location_code: i32,
    /// ISO 639-1 language code, e.g. "en".
    pub language_code: String,
    /// Keywords to forecast traffic for (up to 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// Maximum custom bid, in USD; required by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid: Option<f32>,
    /// Match type: `exact`, `broad` or `phrase`; required by the API.
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
    /// Target domain or URL.
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
    /// `next_week`, `next_month` or `next_quarter`; alternative to `date_from`/`date_to`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_interval: Option<String>,
    /// Start of the forecast range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End of the forecast range, in "yyyy-mm-dd" format.
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

impl KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest {
    /// Creates a request for the given language and location codes.
    pub fn new(language_code: String, location_code: i32) -> Self {
        KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest {
            language_code,
            location_code,
            ..KeywordsDataApiGoogleAdsAdTrafficByKeywordsTaskPostRequest::default()
        }
    }
}
