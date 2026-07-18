use crate::entity::{
    SerpApiElementRefinementChips, SerpApiGoogleOrganicItem, SerpApiGoogleOrganicTaskSpell,
};
use serde::{Deserialize, Serialize};

/// Bing Organic Task Regular SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiBingOrganicTaskRegular {
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
    pub se_results_count: Option<i32>,
    /// Number of items returned in this result.
    pub items_count: Option<i32>,
    // pub items: Option<Vec<Value>>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiGoogleOrganicItem>>,
}
/// Bing Organic Task Advanced SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiBingOrganicTaskAdvanced {
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
    pub se_results_count: Option<i32>,
    /// Number of items returned in this result.
    pub items_count: Option<i32>,
    // pub items: Option<Vec<Value>>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiGoogleOrganicItem>>,
}
