use crate::entity::SerpApiRectangle;
use crate::entity::serp::element::hotels_pack_element::SerpApiElementHotelsPackElement;
use serde::{Deserialize, Serialize};

/// HotelsPack
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementHotelsPack {
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
    /// Date from.
    pub date_from: Option<String>,
    /// Date to.
    pub date_to: Option<String>,
    /// Parsed elements of the result.
    pub items: Option<SerpApiElementHotelsPackElement>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
