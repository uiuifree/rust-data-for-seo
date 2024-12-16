use crate::entity::SerpApiElementLinkElement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFaqBoxElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub links: Option<Vec<SerpApiElementLinkElement>>,
}
