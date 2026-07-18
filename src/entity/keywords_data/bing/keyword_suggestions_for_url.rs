use serde::{Deserialize, Serialize};

/// A result item from the Bing `keyword_suggestions_for_url` endpoint.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/keyword_suggestions_for_url/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingKeywordSuggestionForUrl {
    /// The suggested keyword phrase.
    pub keyword: Option<String>,
    /// Relevance probability from 0.0 to 1.0; higher is more relevant.
    pub confidence_score: Option<f32>,
}
