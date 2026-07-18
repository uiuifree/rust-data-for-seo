//! Entities for Google Hotel Info (`task_get/advanced` / `live/advanced`).
//! See <https://docs.dataforseo.com/v3/business_data/google/hotel_info/task_get/advanced/>.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of a Hotel Info request: a single hotel's detailed profile.
///
/// The `about`, `location`, `reviews`, and `prices` blocks are deeply nested and
/// vary by hotel, so they are surfaced as raw [`Value`] rather than typed structs.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiHotelInfoResult {
    /// Hotel name or identifier the task was set for.
    pub keyword: Option<String>,
    /// Result type, e.g. `"hotel_info"`.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Search engine domain the data was collected from (e.g. `"google.com"`).
    pub se_domain: Option<String>,
    /// Location code the task was run for.
    pub location_code: Option<i32>,
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
    /// Number of hotels in `items` (typically one).
    pub items_count: Option<i64>,
    /// Parsed hotel detail elements.
    pub items: Option<Vec<BusinessDataApiHotelInfoItem>>,
}

impl BusinessDataApiHotelInfoResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiHotelInfoItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// Detailed information about a single hotel.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiHotelInfoItem {
    /// Element type, e.g. `"hotel_info"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Unique Google identifier for the hotel.
    pub hotel_identifier: Option<String>,
    /// Hotel name.
    pub title: Option<String>,
    /// Star classification of the hotel (1-5).
    pub stars: Option<i64>,
    /// Textual description of the star rating.
    pub stars_description: Option<String>,
    /// Full postal address of the hotel.
    pub address: Option<String>,
    /// Hotel phone number.
    pub phone: Option<String>,
    /// Descriptive information and amenities (raw object).
    pub about: Option<Value>,
    /// Geographic details and location scores (raw object).
    pub location: Option<Value>,
    /// Aggregated review data and rating breakdown (raw object).
    pub reviews: Option<Value>,
    /// Preview image URLs for the hotel (raw objects).
    pub overview_images: Option<Vec<Value>>,
    /// Pricing details across booking providers (raw object).
    pub prices: Option<Value>,
}
