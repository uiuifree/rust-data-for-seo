use crate::entity::serp::element::SerpApiElementGoogleJobsItem;
use crate::entity::{SerpApiElementRefinementChips, SerpApiHtmlItem};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleJob<T> {
    pub keyword: String,
    #[serde(rename = "type")]
    pub search_engine_type: Option<String>,
    pub se_domain: Option<String>,
    pub location_code: i32,
    pub language_code: Option<String>,
    pub check_url: Option<String>,
    pub datetime: Option<String>,
    pub spell: Option<Value>,
    pub refinement_chips: Option<SerpApiElementRefinementChips>,
    pub item_types: Option<Vec<String>>,
    pub se_results_count: Option<i32>,
    pub items_count: Option<i32>,
    pub items: Option<Vec<T>>,
}

pub type SerpApiGoogleJobAdvanced = SerpApiGoogleJob<SerpApiElementGoogleJobsItem>;
pub type SerpApiGoogleJobHtml = SerpApiGoogleJob<SerpApiHtmlItem>;
