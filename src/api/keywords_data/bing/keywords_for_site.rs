use crate::DataForSeoApiResponse;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::api::keywords_data::bing::KeywordsDataApiBing;
use crate::entity::KeywordsDataApiBingKeyword;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Bing Keywords For Site.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/keywords_for_site/live/>.
impl KeywordsDataApiBing<'_> {
    /// Posts a Keywords For Site task for asynchronous processing.
    pub async fn keywords_for_site_task_post(
        &self,
        data: Vec<KeywordsDataApiBingKeywordsForSiteTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/keywords_data/bing/keywords_for_site/task_post", &data)
            .await
    }
    /// Lists completed Keywords For Site tasks ready to be collected.
    pub async fn keywords_for_site_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("bing/keywords_for_site")
            .await
    }
    /// Collects the result of a previously posted Keywords For Site task.
    pub async fn keywords_for_site_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingKeyword> {
        self.client
            .http_get(&format!(
                "/v3/keywords_data/bing/keywords_for_site/task_get/{id}"
            ))
            .await
    }
    /// Retrieves Keywords For Site data synchronously.
    pub async fn keywords_for_site_live(
        &self,
        data: Vec<KeywordsDataApiBingKeywordsForSiteTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingKeyword> {
        self.client
            .http_post("/v3/keywords_data/bing/keywords_for_site/live", &data)
            .await
    }
}

/// Request body for the Bing Keywords For Site endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingKeywordsForSiteTaskPostRequest {
    /// Target domain or URL to source keywords from.
    pub target: String,
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

impl KeywordsDataApiBingKeywordsForSiteTaskPostRequest {
    /// Creates a request for the given target domain or URL.
    pub fn new(target: String) -> Self {
        KeywordsDataApiBingKeywordsForSiteTaskPostRequest {
            target,
            ..KeywordsDataApiBingKeywordsForSiteTaskPostRequest::default()
        }
    }
}
