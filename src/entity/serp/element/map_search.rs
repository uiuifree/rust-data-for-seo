use crate::entity::{SerpApiRating, SerpApiRatingDistribution, SerpApiRectangle};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `maps_search` item returned by the Google Maps SERP endpoint.
/// See <https://docs.dataforseo.com/v3/serp/google/maps/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiElementMapsSearch {
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
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// Category of the entity.
    pub category: Option<String>,
    /// Identifiers of the entity's categories.
    pub category_ids: Option<Vec<String>>,
    /// Secondary categories of the entity.
    pub additional_categories: Option<Vec<String>>,
    /// Google CID identifier of the entity.
    pub cid: Option<String>,
    /// Google feature identifier of the entity.
    pub feature_id: Option<String>,
    /// Google Place identifier of the entity.
    pub place_id: Option<String>,
    /// Postal address of the entity.
    pub address: Option<String>,
    /// Structured address details of the entity.
    pub address_info: Option<Value>,
    /// Phone number of the entity.
    pub phone: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// URL to contact the entity.
    pub contact_url: Option<String>,
    /// URL of the contributor.
    pub contributor_url: Option<String>,
    /// URL to book the entity online.
    pub book_online_url: Option<String>,
    /// URL of the entity's primary image.
    pub main_image: Option<String>,
    /// Total number of photos on the profile.
    pub total_photos: Option<i64>,
    /// Snippet text of the result.
    pub snippet: Option<String>,
    /// Latitude of the entity.
    pub latitude: Option<f64>,
    /// Longitude of the entity.
    pub longitude: Option<f64>,
    /// `true` if the business profile is claimed by its owner.
    pub is_claimed: Option<bool>,
    /// Relative price level of the entity.
    pub price_level: Option<String>,
    /// Opening-hours information for the entity.
    pub work_hours: Option<Value>,
    /// Aggregate rating shown for the result.
    pub rating: Option<SerpApiRating>,
    /// Breakdown of ratings by star value.
    pub rating_distribution: Option<SerpApiRatingDistribution>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
