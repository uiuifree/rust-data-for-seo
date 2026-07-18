use crate::entity::{SerpApiElementCarouselElement, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// Carousel
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementCarousel {
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Alignment of the element within the SERP, `left` or `right`.
    pub position: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiElementCarouselElement>>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
