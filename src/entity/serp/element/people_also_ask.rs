use crate::entity::{SerpApiElementPeopleAlsoAskElement, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// PeopleAlsoAsk
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementPeopleAlsoAsk {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub items: Option<Vec<SerpApiElementPeopleAlsoAskElement>>,
    pub rectangle: Option<SerpApiRectangle>,
}
