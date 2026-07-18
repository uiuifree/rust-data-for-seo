//! Entities for Google My Business Info (`task_get` / `live`).
//! See <https://docs.dataforseo.com/v3/business_data/google/my_business_info/task_get/>.

use crate::entity::{BusinessDataApiAddressInfo, BusinessDataApiRating};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of a My Business Info request: search metadata plus its `items`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiMyBusinessInfoResult {
    /// Business name or identifier the task was set for.
    pub keyword: Option<String>,
    /// Result type, e.g. `"my_business_info"`.
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
    /// Distinct element types present in `items`.
    pub item_types: Option<Vec<String>>,
    /// Number of elements in `items`.
    pub items_count: Option<i64>,
    /// Parsed business profile elements.
    pub items: Option<Vec<BusinessDataApiBusinessItem>>,
}

impl BusinessDataApiMyBusinessInfoResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiBusinessItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// A Google Business Profile item, shared by My Business Info and Business Listings.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiBusinessItem {
    /// Element type, e.g. `"business_info"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank within elements of the same type.
    pub rank_group: Option<i64>,
    /// Rank across all elements regardless of type.
    pub rank_absolute: Option<i64>,
    /// Placement of the element on the page (e.g. `"left"`).
    pub position: Option<String>,
    /// Business name.
    pub title: Option<String>,
    /// Original, untranslated business name.
    pub original_title: Option<String>,
    /// Business description.
    pub description: Option<String>,
    /// Primary business category.
    pub category: Option<String>,
    /// Identifiers of the categories the business belongs to.
    pub category_ids: Option<Vec<String>>,
    /// Additional categories beyond the primary one.
    pub additional_categories: Option<Vec<String>>,
    /// Google-defined customer identifier (CID) for the business.
    pub cid: Option<String>,
    /// Unique Google feature identifier for the business.
    pub feature_id: Option<String>,
    /// Full postal address as a single string.
    pub address: Option<String>,
    /// Structured breakdown of the address.
    pub address_info: Option<BusinessDataApiAddressInfo>,
    /// Google Place identifier for the business.
    pub place_id: Option<String>,
    /// Business phone number.
    pub phone: Option<String>,
    /// Business website URL.
    pub url: Option<String>,
    /// URL of the business contact page.
    pub contact_url: Option<String>,
    /// URL to suggest edits to the profile.
    pub contributor_url: Option<String>,
    /// URL to book the business online.
    pub book_online_url: Option<String>,
    /// Domain of the business website.
    pub domain: Option<String>,
    /// URL of the business logo image.
    pub logo: Option<String>,
    /// URL of the main business image.
    pub main_image: Option<String>,
    /// Total number of photos on the profile.
    pub total_photos: Option<i64>,
    /// Short text snippet describing the business.
    pub snippet: Option<String>,
    /// Latitude of the business location.
    pub latitude: Option<f64>,
    /// Longitude of the business location.
    pub longitude: Option<f64>,
    /// Whether the business profile has been claimed by its owner.
    pub is_claimed: Option<bool>,
    /// Business attributes grouped by category (raw object).
    pub attributes: Option<Value>,
    /// Topics mentioned in reviews, with mention counts (raw object).
    pub place_topics: Option<Value>,
    /// Aggregated rating of the business.
    pub rating: Option<BusinessDataApiRating>,
    /// Vote counts per star level, keyed `"1"`..`"5"` (raw object).
    pub rating_distribution: Option<Value>,
    /// Businesses people also searched for (raw objects).
    pub people_also_search: Option<Vec<Value>>,
    /// Opening hours and current open/closed status (raw object).
    pub work_time: Option<Value>,
    /// Historical foot-traffic data by day and hour (raw object).
    pub popular_times: Option<Value>,
    /// Links to related business resources (raw objects).
    pub local_business_links: Option<Vec<Value>>,
    /// Whether the element is a directory sub-listing.
    pub is_directory_item: Option<bool>,
    /// Directory listing details (raw object).
    pub directory: Option<Value>,
    /// Price level indicator (e.g. `"$$"`).
    pub price_level: Option<String>,
    /// Hotel class rating, for lodging businesses.
    pub hotel_rating: Option<i64>,
    /// Services offered by the business (raw object).
    pub services: Option<Value>,
    /// UTC time the listing was last updated in the database.
    pub last_updated_time: Option<String>,
    /// UTC time the listing was first seen in the database.
    pub first_seen: Option<String>,
}
