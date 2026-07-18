use crate::DataForSeoApiResponse;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::api::keywords_data::bing::KeywordsDataApiBing;
use crate::entity::KeywordsDataApiBingKeyword;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Bing Search Volume.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/search_volume/live/>.
impl KeywordsDataApiBing<'_> {
    /// Posts a Search Volume task for asynchronous processing.
    pub async fn search_volume_task_post(
        &self,
        data: Vec<KeywordsDataApiBingSearchVolumeTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/keywords_data/bing/search_volume/task_post", &data)
            .await
    }
    /// Lists completed Search Volume tasks ready to be collected.
    pub async fn search_volume_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("bing/search_volume")
            .await
    }
    /// Collects the result of a previously posted Search Volume task.
    pub async fn search_volume_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingKeyword> {
        self.client
            .http_get(&format!(
                "/v3/keywords_data/bing/search_volume/task_get/{id}"
            ))
            .await
    }
    /// Retrieves Search Volume data synchronously.
    pub async fn search_volume_live(
        &self,
        data: Vec<KeywordsDataApiBingSearchVolumeTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingKeyword> {
        self.client
            .http_post("/v3/keywords_data/bing/search_volume/live", &data)
            .await
    }
}

/// Request body for the Bing Search Volume endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingSearchVolumeTaskPostRequest {
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
    /// Device filter: "all", "mobile", "desktop" or "tablet".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Include Bing search partner data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_partners: Option<bool>,
    /// Start of the historical range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End of the historical range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Field to sort by: "search_volume", "cpc", "competition" or "relevance".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiBingSearchVolumeTaskPostRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> Self {
        KeywordsDataApiBingSearchVolumeTaskPostRequest {
            keywords,
            ..KeywordsDataApiBingSearchVolumeTaskPostRequest::default()
        }
    }
}
