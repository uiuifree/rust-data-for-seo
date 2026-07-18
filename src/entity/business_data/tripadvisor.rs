//! Entities for the Tripadvisor Search and Reviews endpoints.
//! See <https://docs.dataforseo.com/v3/business_data/tripadvisor/overview/>.

use crate::entity::{BusinessDataApiAddressInfo, BusinessDataApiRating};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Task request for the Tripadvisor Search `task_post` endpoint.
/// See <https://docs.dataforseo.com/v3/business_data/tripadvisor/search/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTripadvisorSearchTaskPost {
    /// Business name or search query to look up on Tripadvisor.
    pub keyword: String,
    /// Full location name, e.g. `"London,England,United Kingdom"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code from the `locations` endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full language name, e.g. `"English"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code, e.g. `"en"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Business category to restrict the search to (e.g. `"hotels"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Task priority: `1` (normal, default) or `2` (high, extra cost).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL that receives the finished results via POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// URL notified via GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl BusinessDataApiTripadvisorSearchTaskPost {
    /// Builds a Tripadvisor search task request for the given keyword.
    pub fn new<T: Into<String>>(keyword: T) -> Self {
        BusinessDataApiTripadvisorSearchTaskPost {
            keyword: keyword.into(),
            ..BusinessDataApiTripadvisorSearchTaskPost::default()
        }
    }
}

/// Task request for the Tripadvisor Reviews `task_post` endpoint.
/// See <https://docs.dataforseo.com/v3/business_data/tripadvisor/reviews/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTripadvisorReviewsTaskPost {
    /// Business name or Tripadvisor URL path identifying the entity.
    pub keyword: String,
    /// Full location name, e.g. `"London,England,United Kingdom"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// Location code from the `locations` endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<i32>,
    /// Full language name, e.g. `"English"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code, e.g. `"en"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Number of reviews to parse (default `10`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Review ordering, e.g. `"most_recent"` or `"most_helpful"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Task priority: `1` (normal, default) or `2` (high, extra cost).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// URL that receives the finished results via POST.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postback_url: Option<String>,
    /// URL notified via GET when the task completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pingback_url: Option<String>,
}

impl BusinessDataApiTripadvisorReviewsTaskPost {
    /// Builds a Tripadvisor reviews task request for the given keyword or URL path.
    pub fn new<T: Into<String>>(keyword: T) -> Self {
        BusinessDataApiTripadvisorReviewsTaskPost {
            keyword: keyword.into(),
            ..BusinessDataApiTripadvisorReviewsTaskPost::default()
        }
    }
}

/// Result of a Tripadvisor Search request: metadata plus matched businesses.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTripadvisorSearchResult {
    /// Business name or query the task was set for.
    pub keyword: Option<String>,
    /// Result type, e.g. `"tripadvisor_search"`.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Source domain the data was collected from (e.g. `"tripadvisor.com"`).
    pub se_domain: Option<String>,
    /// Location code the task was run for.
    pub location_code: Option<i32>,
    /// Language code the task was run for.
    pub language_code: Option<String>,
    /// URL of the Tripadvisor results page the data was parsed from.
    pub check_url: Option<String>,
    /// UTC time the result was collected.
    pub datetime: Option<String>,
    /// Number of businesses in `items`.
    pub items_count: Option<i64>,
    /// Parsed business listings.
    pub items: Option<Vec<BusinessDataApiTripadvisorSearchItem>>,
}

impl BusinessDataApiTripadvisorSearchResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiTripadvisorSearchItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// A single business listing returned by Tripadvisor Search.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTripadvisorSearchItem {
    /// Element type, e.g. `"tripadvisor_search_organic"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank within elements of the same type.
    pub rank_group: Option<i64>,
    /// Rank across all elements regardless of type.
    pub rank_absolute: Option<i64>,
    /// Business name.
    pub title: Option<String>,
    /// Business category on Tripadvisor.
    pub category: Option<String>,
    /// Business description.
    pub description: Option<String>,
    /// Full URL of the business's Tripadvisor page.
    pub url: Option<String>,
    /// Path portion of the business's Tripadvisor page.
    pub url_path: Option<String>,
    /// Business website domain.
    pub domain: Option<String>,
    /// Whether the listing is a sponsored placement.
    pub is_sponsored: Option<bool>,
    /// Full postal address as a single string.
    pub address: Option<String>,
    /// Structured breakdown of the address.
    pub address_info: Option<BusinessDataApiAddressInfo>,
    /// Latitude of the business location.
    pub latitude: Option<f64>,
    /// Longitude of the business location.
    pub longitude: Option<f64>,
    /// Business phone number.
    pub phone: Option<String>,
    /// URL of the main business image.
    pub main_image: Option<String>,
    /// Price level indicator (e.g. `"$$"`).
    pub price_level: Option<String>,
    /// Price range shown on the listing (e.g. `"$$ - $$$"`).
    pub price_rate: Option<String>,
    /// Hotel class rating, for lodging businesses.
    pub hotel_class: Option<f64>,
    /// Total number of reviews the business has received.
    pub reviews_count: Option<i64>,
    /// Aggregated Tripadvisor rating.
    pub rating: Option<BusinessDataApiRating>,
    /// Vote counts per rating level (raw object).
    pub rating_distribution: Option<Value>,
}

/// Result of a Tripadvisor Reviews request: metadata plus individual reviews.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTripadvisorReviewsResult {
    /// Business name or URL path the task was set for.
    pub keyword: Option<String>,
    /// Result type, e.g. `"tripadvisor_reviews"`.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Source domain the data was collected from (e.g. `"tripadvisor.com"`).
    pub se_domain: Option<String>,
    /// Location code the task was run for.
    pub location_code: Option<i32>,
    /// Language code the task was run for.
    pub language_code: Option<String>,
    /// URL of the Tripadvisor business page the data was parsed from.
    pub check_url: Option<String>,
    /// UTC time the result was collected.
    pub datetime: Option<String>,
    /// Business name the reviews belong to.
    pub title: Option<String>,
    /// Total number of reviews the business has received.
    pub reviews_count: Option<i64>,
    /// Aggregated Tripadvisor rating.
    pub rating: Option<BusinessDataApiRating>,
    /// Number of reviews in `items`.
    pub items_count: Option<i64>,
    /// Parsed individual reviews.
    pub items: Option<Vec<BusinessDataApiTripadvisorReviewItem>>,
}

impl BusinessDataApiTripadvisorReviewsResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiTripadvisorReviewItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// A single Tripadvisor review.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTripadvisorReviewItem {
    /// Element type, e.g. `"tripadvisor_review"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank within elements of the same type.
    pub rank_group: Option<i64>,
    /// Rank across all elements regardless of type.
    pub rank_absolute: Option<i64>,
    /// Placement of the element on the page.
    pub position: Option<String>,
    /// Unique identifier of the review.
    pub review_id: Option<String>,
    /// Direct URL to the review.
    pub url: Option<String>,
    /// Review headline.
    pub title: Option<String>,
    /// Review body text.
    pub review_text: Option<String>,
    /// Language code of the review.
    pub language: Option<String>,
    /// UTC timestamp the review was published.
    pub timestamp: Option<String>,
    /// Date of the visit the review describes.
    pub date_of_visit: Option<String>,
    /// Rating given by this review.
    pub rating: Option<BusinessDataApiRating>,
    /// Reviewer profile details (raw object).
    pub user_profile: Option<Value>,
    /// Images attached to the review (raw objects).
    pub review_images: Option<Vec<Value>>,
    /// Owner replies to the review (raw objects).
    pub responses: Option<Vec<Value>>,
}
