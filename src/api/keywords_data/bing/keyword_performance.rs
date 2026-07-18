use crate::DataForSeoApiResponse;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::api::keywords_data::bing::KeywordsDataApiBing;
use crate::entity::KeywordsDataApiBingKeywordPerformance;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Bing Keyword Performance.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/keyword_performance/live/>.
impl KeywordsDataApiBing<'_> {
    /// Posts a Keyword Performance task for asynchronous processing.
    pub async fn keyword_performance_task_post(
        &self,
        data: Vec<KeywordsDataApiBingKeywordPerformanceTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "/v3/keywords_data/bing/keyword_performance/task_post",
                &data,
            )
            .await
    }
    /// Lists completed Keyword Performance tasks ready to be collected.
    pub async fn keyword_performance_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("bing/keyword_performance")
            .await
    }
    /// Collects the result of a previously posted Keyword Performance task.
    pub async fn keyword_performance_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingKeywordPerformance> {
        self.client
            .http_get(&format!(
                "/v3/keywords_data/bing/keyword_performance/task_get/{id}"
            ))
            .await
    }
    /// Retrieves Keyword Performance data synchronously.
    pub async fn keyword_performance_live(
        &self,
        data: Vec<KeywordsDataApiBingKeywordPerformanceTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingKeywordPerformance> {
        self.client
            .http_post("/v3/keywords_data/bing/keyword_performance/live", &data)
            .await
    }
}

/// Request body for the Bing Keyword Performance endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingKeywordPerformanceTaskPostRequest {
    /// Keywords to estimate performance for.
    pub keywords: Vec<String>,
    /// Device filter: "all", "mobile", "desktop" or "tablet".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Match type: `exact`, `broad` or `phrase`.
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub match_type: Option<String>,
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
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiBingKeywordPerformanceTaskPostRequest {
    /// Creates a request for the given keywords.
    pub fn new(keywords: Vec<String>) -> Self {
        KeywordsDataApiBingKeywordPerformanceTaskPostRequest {
            keywords,
            ..KeywordsDataApiBingKeywordPerformanceTaskPostRequest::default()
        }
    }
}
