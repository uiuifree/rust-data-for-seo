//! Entities for Google Hotel Searches (`task_get` / `live`).
//! See <https://docs.dataforseo.com/v3/business_data/google/hotel_searches/task_get/>.

use crate::entity::BusinessDataApiRating;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of a Hotel Searches request: metadata plus hotel listings.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiHotelSearchesResult {
    /// Search query the task was set for, with location appended.
    pub keyword: Option<String>,
    /// Result type, e.g. `"hotel_searches"`.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Search engine domain the data was collected from (e.g. `"google.com"`).
    pub se_domain: Option<String>,
    /// Location code the task was run for.
    pub location_code: Option<i64>,
    /// Language code the task was run for.
    pub language_code: Option<String>,
    /// URL of the Google results page the data was parsed from.
    pub check_url: Option<String>,
    /// UTC time the result was collected.
    pub datetime: Option<String>,
    /// Check-in date the prices apply to, in `yyyy-mm-dd` format.
    pub check_in: Option<String>,
    /// Check-out date the prices apply to, in `yyyy-mm-dd` format.
    pub check_out: Option<String>,
    /// Number of hotels in `items`.
    pub items_count: Option<i64>,
    /// Parsed hotel listings.
    pub items: Option<Vec<BusinessDataApiHotelSearchItem>>,
}

impl BusinessDataApiHotelSearchesResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiHotelSearchItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// A single hotel returned by Hotel Searches.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiHotelSearchItem {
    /// Element type, e.g. `"hotel_search_item"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank within elements of the same type.
    pub rank_group: Option<i64>,
    /// Rank across all elements regardless of type.
    pub rank_absolute: Option<i64>,
    /// Hotel name.
    pub title: Option<String>,
    /// Unique Google identifier for the hotel.
    pub hotel_identifier: Option<String>,
    /// Star classification of the hotel (1-5).
    pub stars: Option<i64>,
    /// Whether the listing is a sponsored placement.
    pub is_paid: Option<bool>,
    /// GPS coordinates of the hotel (raw object).
    pub location: Option<Value>,
    /// Aggregated guest review rating.
    pub reviews: Option<BusinessDataApiRating>,
    /// Preview image URLs for the hotel (raw objects).
    pub overview_images: Option<Vec<Value>>,
    /// Pricing details across booking providers (raw object).
    pub prices: Option<Value>,
    /// Criterion-based review breakdown; `null` for this endpoint.
    pub mentions: Option<Value>,
    /// Vote counts per star level; `null` for this endpoint.
    pub rating_distribution: Option<Value>,
    /// Third-party site review citations; `null` for this endpoint.
    pub other_sites_reviews: Option<Value>,
}
