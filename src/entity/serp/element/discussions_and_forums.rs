use crate::entity::{SerpApiElementDiscussionsAndForumsElement, SerpApiRectangle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDiscussionsAndForums {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,

    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,

    pub position: Option<String>,
    pub xpath: Option<String>,
    pub title: Option<String>,
    pub items: Option<Vec<SerpApiElementDiscussionsAndForumsElement>>,
    pub rectangle: Option<SerpApiRectangle>,
}
