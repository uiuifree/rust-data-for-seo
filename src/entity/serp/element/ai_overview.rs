use crate::entity::SerpApiElementAiOverviewElement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAiOverview {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub asynchronous_ai_overview: Option<bool>,
    pub items: Option<Vec<SerpApiElementAiOverviewElement>>,
}
