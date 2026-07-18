use crate::entity::{SerpApiElementDiscussionsAndForumsElement, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// Discussions And Forums SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDiscussionsAndForums {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,

    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i64>,

    /// Alignment of the element within the SERP, `left` or `right`.
    pub position: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiElementDiscussionsAndForumsElement>>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
