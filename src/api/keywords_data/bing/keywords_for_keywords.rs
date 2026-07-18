use crate::DataForSeoApiResponse;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::api::keywords_data::bing::KeywordsDataApiBing;
use crate::entity::KeywordsDataApiBingKeyword;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Bing Keywords For Keywords.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/keywords_for_keywords/live/>.
impl KeywordsDataApiBing<'_> {
    /// Posts a Keywords For Keywords task for asynchronous processing.
    pub async fn keywords_for_keywords_task_post(
        &self,
        data: Vec<KeywordsDataApiBingKeywordsForKeywordsTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "/v3/keywords_data/bing/keywords_for_keywords/task_post",
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
            .task_ready_se("bing/keywords_for_keywords")
            .await
    }
    /// Collects the result of a previously posted Keywords For Keywords task.
    pub async fn keywords_for_keywords_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingKeyword> {
        self.client
            .http_get(&format!(
                "/v3/keywords_data/bing/keywords_for_keywords/task_get/{id}"
            ))
            .await
    }
    /// Retrieves Keywords For Keywords data synchronously.
    pub async fn keywords_for_keywords_live(
        &self,
        data: Vec<KeywordsDataApiBingKeywordsForKeywordsTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingKeyword> {
        self.client
            .http_post("/v3/keywords_data/bing/keywords_for_keywords/live", &data)
            .await
    }
}

/// Request body for the Bing Keywords For Keywords endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingKeywordsForKeywordsTaskPostRequest {
    /// Seed keywords to expand.
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
    /// Keywords to exclude from the results (up to 200).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords_negative: Option<Vec<String>>,
    /// Device filter: "all", "mobile", "desktop" or "tablet".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Start of the historical range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End of the historical range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Field to sort by: "search_volume", "cpc", "competition" or "relevance".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Include Bing search partner data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_partners: Option<bool>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiBingKeywordsForKeywordsTaskPostRequest {
    /// Creates a request for the given seed keywords.
    pub fn new(keywords: Vec<String>) -> Self {
        KeywordsDataApiBingKeywordsForKeywordsTaskPostRequest {
            keywords,
            ..KeywordsDataApiBingKeywordsForKeywordsTaskPostRequest::default()
        }
    }
}
