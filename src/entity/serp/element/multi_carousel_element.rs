use crate::entity::SerpApiElementMultiCarouselSnippet;
use serde::{Deserialize, Serialize};

/// Multi Carousel Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementMultiCarouselElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Multi carousel snippets.
    pub multi_carousel_snippets: Option<Vec<SerpApiElementMultiCarouselSnippet>>,
}
