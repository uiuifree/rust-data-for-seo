use crate::DataForSeoApiResponse;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::api::keywords_data::google_trends::KeywordsDataApiGoogleTrends;
use crate::entity::KeywordsDataApiGoogleTrendsExplore;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Trends Explore.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_trends/explore/live/>.
impl KeywordsDataApiGoogleTrends<'_> {
    /// Posts an Explore task for asynchronous processing.
    pub async fn explore_task_post(
        &self,
        data: Vec<KeywordsDataApiGoogleTrendsExploreTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post("/v3/keywords_data/google_trends/explore/task_post", &data)
            .await
    }
    /// Lists completed Explore tasks ready to be collected.
    pub async fn explore_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("google_trends/explore")
            .await
    }
    /// Collects the result of a previously posted Explore task.
    pub async fn explore_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleTrendsExplore> {
        self.client
            .http_get(&format!(
                "/v3/keywords_data/google_trends/explore/task_get/{id}"
            ))
            .await
    }
    /// Retrieves Explore data synchronously.
    pub async fn explore_live(
        &self,
        data: Vec<KeywordsDataApiGoogleTrendsExploreTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiGoogleTrendsExplore> {
        self.client
            .http_post("/v3/keywords_data/google_trends/explore/live", &data)
            .await
    }
}

/// Request body for the Google Trends Explore endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsExploreTaskPostRequest {
    /// Keywords to explore (up to 5).
    pub keywords: Vec<String>,
    /// Full location name; alternative to `location_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Numeric location identifier; alternative to `location_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full language name; alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code, e.g. "en"; alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Trends data source: `web`, `news`, `youtube`, `images` or `froogle`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub explore_type: Option<String>,
    /// Google Trends category code to narrow results (0 = all categories).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_code: Option<i32>,
    /// Start of the range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_from: Option<String>,
    /// End of the range, in "yyyy-mm-dd" format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_to: Option<String>,
    /// Preset range, e.g. "past_7_days" or "past_12_months"; alternative to dates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<String>,
    /// Which elements to return, e.g. "google_trends_graph" or "google_trends_map".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_types: Option<Vec<String>>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiGoogleTrendsExploreTaskPostRequest {
    /// Creates a request for the given keywords (max 5).
    pub fn new(keywords: Vec<String>) -> Self {
        KeywordsDataApiGoogleTrendsExploreTaskPostRequest {
            keywords,
            ..KeywordsDataApiGoogleTrendsExploreTaskPostRequest::default()
        }
    }
}
