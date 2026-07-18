use crate::entity::{SerpApiElementMultiCarouselElement, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// MultiCarousel
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementMultiCarousel {
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i64>,
    /// Alignment of the element within the SERP, `left` or `right`.
    pub position: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiElementMultiCarouselElement>>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
