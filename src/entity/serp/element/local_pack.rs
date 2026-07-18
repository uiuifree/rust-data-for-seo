use crate::entity::{SerpApiRating, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// LocalPack
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementLocalPack {
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
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Phone number of the entity.
    pub phone: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// URL to make a booking or reservation.
    pub booking_url: Option<String>,
    /// `true` if the result is a paid placement.
    pub is_paid: Option<String>,
    /// Aggregate rating shown for the result.
    pub rating: Option<SerpApiRating>,
    /// Google CID identifier of the entity.
    pub cid: Option<String>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
