use crate::entity::serp::element::{
    SerpApiElementGoogleNewsSearch, SerpApiGoogleOrganicItemTopStories,
};
use crate::entity::{
    SerpApiElementRefinementChips, SerpApiGoogleOrganicTaskSpell, SerpApiHtmlItem,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google News Task SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleNewsTask<T> {
    /// Search term the result was returned for.
    pub keyword: String,
    /// Search engine result type (the API `type` field).
    #[serde(rename = "type")]
    pub search_engine_type: Option<String>,
    /// Search-engine domain the results were taken from.
    pub se_domain: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: i32,
    /// Language code the search was run for.
    pub language_code: Option<String>,
    /// Direct URL to reproduce the search on the search engine.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was received.
    pub datetime: Option<String>,
    /// Search-engine spelling correction applied to the query, if any.
    pub spell: Option<SerpApiGoogleOrganicTaskSpell>,
    /// Search-refinement chips shown for the query.
    pub refinement_chips: Option<SerpApiElementRefinementChips>,
    /// Distinct element types present in the returned SERP.
    pub item_types: Option<Vec<String>>,
    /// Total number of results reported by the search engine.
    pub se_results_count: Option<i64>,
    /// Number of items returned in this result.
    pub items_count: Option<i64>,
    /// Parsed elements of the result.
    pub items: Option<Vec<T>>,
}

/// Google News Task Advanced result type.
pub type SerpApiGoogleNewsTaskAdvanced = SerpApiGoogleNewsTask<SerpApiGoogleNewsItem>;
/// Google News Task Html result type.
pub type SerpApiGoogleNewsTaskHtml = SerpApiGoogleNewsTask<SerpApiHtmlItem>;

/// Google News Item: a SERP element tagged by the DataForSEO `type` field.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SerpApiGoogleNewsItem {
    /// Element of type `news_search`.
    #[serde(rename = "news_search")]
    NewsSearch(SerpApiElementGoogleNewsSearch),
    /// Element of type `top_stories`.
    #[serde(rename = "top_stories")]
    TopStories(SerpApiGoogleOrganicItemTopStories),
    /// Fallback holding the raw JSON of an unrecognized `type`.
    #[serde(untagged)]
    Unknown(Value),
}
