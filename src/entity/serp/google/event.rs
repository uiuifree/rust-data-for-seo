use crate::entity::serp::element::SerpApiElementEventItem;
use crate::entity::{SerpApiElementRefinementChips, SerpApiGoogleOrganicTaskSpell};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleEventAdvance {
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
    pub se_results_count: Option<i32>,
    pub items_count: Option<i32>,
    pub items: Option<Vec<SerpApiElementEventItem>>,
}
