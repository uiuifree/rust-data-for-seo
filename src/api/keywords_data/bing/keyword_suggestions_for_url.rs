use crate::DataForSeoApiResponse;
use crate::api::keywords_data::KeywordsDataApiTaskReadyResult;
use crate::api::keywords_data::bing::KeywordsDataApiBing;
use crate::entity::{KeywordsDataApiBingKeywordSuggestionForUrl, KeywordsDataApiLanguage};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Bing Keyword Suggestions For URL.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/keyword_suggestions_for_url/live/>.
impl KeywordsDataApiBing<'_> {
    /// Languages supported by the Keyword Suggestions For URL endpoint.
    /// See <https://docs.dataforseo.com/v3/keywords_data/bing/keyword_suggestions_for_url/languages/>.
    pub async fn keyword_suggestions_for_url_languages(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiLanguage> {
        self.client
            .http_get("/v3/keywords_data/bing/keyword_suggestions_for_url/languages")
            .await
    }
    /// Posts a Keyword Suggestions For URL task for asynchronous processing.
    pub async fn keyword_suggestions_for_url_task_post(
        &self,
        data: Vec<KeywordsDataApiBingKeywordSuggestionsForUrlTaskPostRequest>,
    ) -> DataForSeoApiResponse<Value> {
        self.client
            .http_post(
                "/v3/keywords_data/bing/keyword_suggestions_for_url/task_post",
                &data,
            )
            .await
    }
    /// Lists completed Keyword Suggestions For URL tasks ready to be collected.
    pub async fn keyword_suggestions_for_url_tasks_ready(
        &self,
    ) -> DataForSeoApiResponse<KeywordsDataApiTaskReadyResult> {
        self.client
            .keywords_data()
            .task_ready_se("bing/keyword_suggestions_for_url")
            .await
    }
    /// Collects the result of a previously posted Keyword Suggestions For URL task.
    pub async fn keyword_suggestions_for_url_task_get(
        &self,
        id: &str,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingKeywordSuggestionForUrl> {
        self.client
            .http_get(&format!(
                "/v3/keywords_data/bing/keyword_suggestions_for_url/task_get/{id}"
            ))
            .await
    }
    /// Retrieves Keyword Suggestions For URL data synchronously.
    pub async fn keyword_suggestions_for_url_live(
        &self,
        data: Vec<KeywordsDataApiBingKeywordSuggestionsForUrlTaskPostRequest>,
    ) -> DataForSeoApiResponse<KeywordsDataApiBingKeywordSuggestionForUrl> {
        self.client
            .http_post(
                "/v3/keywords_data/bing/keyword_suggestions_for_url/live",
                &data,
            )
            .await
    }
}

/// Request body for the Bing Keyword Suggestions For URL endpoints.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingKeywordSuggestionsForUrlTaskPostRequest {
    /// Target webpage URL (max 2000 characters).
    pub target: String,
    /// Full language name; alternative to `language_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code, e.g. "en"; alternative to `language_name`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Exclude brand keywords from the suggestions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_brands: Option<bool>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl KeywordsDataApiBingKeywordSuggestionsForUrlTaskPostRequest {
    /// Creates a request for the given target webpage URL.
    pub fn new(target: String) -> Self {
        KeywordsDataApiBingKeywordSuggestionsForUrlTaskPostRequest {
            target,
            ..KeywordsDataApiBingKeywordSuggestionsForUrlTaskPostRequest::default()
        }
    }
}
