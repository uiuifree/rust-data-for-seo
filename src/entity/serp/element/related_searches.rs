use crate::entity::SerpApiRectangle;
use serde::{Deserialize, Serialize};

/// RelatedSearches
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementRelatedSearches {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub items: Option<Vec<String>>,
    pub rectangle: Option<SerpApiRectangle>,
}
