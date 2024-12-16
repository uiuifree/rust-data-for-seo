use crate::entity::SerpApiElementMultiCarouselSnippet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementMultiCarouselElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub title: Option<String>,
    pub multi_carousel_snippets: Option<Vec<SerpApiElementMultiCarouselSnippet>>,
}
