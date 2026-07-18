use crate::DataForSeoApiResponse;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::api::keywords_data::bing::KeywordsDataApiBing;
use crate::entity::{
    KeywordsDataApiBingSearchVolumeHistory, KeywordsDataApiBingSearchVolumeHistoryLocationLanguage,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Bing Search Volume History.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/search_volume_history/live/>.
impl KeywordsDataApiBing<'_> {
    /// Locations and their available languages for Search Volume History.
    /// See <https://docs.dataforseo.com/v3/keywords_data/bing/search_volume_history/locations_and_languages/>.
    pub async fn search_volume_history_locations_and_languages(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingSearchVolumeHistoryLocationLanguage> {
        self.client
            .http_get("/v3/keywords_data/bing/search_volume_history/locations_and_languages")
            .await
    }
    /// Posts a Search Volume History task for asynchronous processing.
    pub async fn search_volume_history_task_post(
        &self,
        data: Vec<KeywordsDataApiBingSearchVolumeHistoryTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "/v3/keywords_data/bing/search_volume_history/task_post",
                &data,
            )
            .await
    }
    /// Lists completed Search Volume History tasks ready to be collected.
    pub async fn search_volume_history_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("bing/search_volume_history")
            .await
    }
    /// Collects the result of a previously posted Search Volume History task.
    pub async fn search_volume_history_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingSearchVolumeHistory> {
        self.client
            .http_get(&format!(
                "/v3/keywords_data/bing/search_volume_history/task_get/{id}"
            ))
            .await
    }
    /// Retrieves Search Volume History data synchronously.
    pub async fn search_volume_history_live(
        &self,
        data: Vec<KeywordsDataApiBingSearchVolumeHistoryTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingSearchVolumeHistory> {
        self.client
            .http_post("/v3/keywords_data/bing/search_volume_history/live", &data)
            .await
    }
}

/// Request body for the Bing Search Volume History endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingSearchVolumeHistoryTaskPostRequest {
    /// Keywords to look up (up to 1000 per request).
    pub keywords: Vec<String>,
    /// Full location name; alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Numeric location identifier; alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// GPS coordinates as "latitude,longitude".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_coordinate: Option<String>,
    /// Full language name; alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code, e.g. "en"; alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Device filters: "mobile", "desktop", "tablet" or "non_smartphones".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Vec<String>>,
    /// Data aggregation: "monthly", "weekly" or "daily".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// Start of the historical range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End of the historical range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiBingSearchVolumeHistoryTaskPostRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> Self {
        KeywordsDataApiBingSearchVolumeHistoryTaskPostRequest {
            keywords,
            ..KeywordsDataApiBingSearchVolumeHistoryTaskPostRequest::default()
        }
    }
}
