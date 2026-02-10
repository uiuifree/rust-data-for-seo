use crate::entity::serp::element::{
    SerpApiElementGoogleNewsSearch, SerpApiGoogleOrganicItemTopStories,
};
use crate::entity::{
    SerpApiElementRefinementChips, SerpApiGoogleOrganicTaskSpell, SerpApiHtmlItem,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleNewsTask<T> {
    pub keyword: String,
    #[serde(rename = "type")]
    pub search_engine_type: Option<String>,
    pub se_domain: Option<String>,
    pub location_code: i32,
    pub language_code: Option<String>,
    pub check_url: Option<String>,
    pub datetime: Option<String>,
    pub spell: Option<SerpApiGoogleOrganicTaskSpell>,
    pub refinement_chips: Option<SerpApiElementRefinementChips>,
    pub item_types: Option<Vec<String>>,
    pub se_results_count: Option<i64>,
    pub items_count: Option<i64>,
    pub items: Option<Vec<T>>,
}

pub type SerpApiGoogleNewsTaskAdvanced = SerpApiGoogleNewsTask<SerpApiGoogleNewsItem>;
pub type SerpApiGoogleNewsTaskHtml = SerpApiGoogleNewsTask<SerpApiHtmlItem>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SerpApiGoogleNewsItem {
    #[serde(rename = "news_search")]
    NewsSearch(SerpApiElementGoogleNewsSearch),
    #[serde(rename = "top_stories")]
    TopStories(SerpApiGoogleOrganicItemTopStories),
    #[serde(untagged)]
    Unknown(Value),
}
