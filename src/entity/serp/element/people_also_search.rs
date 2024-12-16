use crate::entity::SerpApiRectangle;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// PeopleAlsoSearch
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementPeopleAlsoSearch {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub title: Option<String>,
    pub items: Option<Value>,
    pub rectangle: Option<SerpApiRectangle>,
}
