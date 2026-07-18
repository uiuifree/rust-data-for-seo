//! Entities for the Trustpilot Search and Reviews endpoints.
//! See <https://docs.dataforseo.com/v3/business_data/trustpilot/overview/>.

use crate::entity::BusinessDataApiRating;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Task request for the Trustpilot Search `task_post` endpoint.
/// See <https://docs.dataforseo.com/v3/business_data/trustpilot/search/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTrustpilotSearchTaskPost {
    /// Business name to search for on Trustpilot.
    pub keyword: String,
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

impl BusinessDataApiTrustpilotSearchTaskPost {
    /// Builds a Trustpilot search task request for the given business name.
    pub fn new<T: Into<String>>(keyword: T) -> Self {
        BusinessDataApiTrustpilotSearchTaskPost {
            keyword: keyword.into(),
            ..BusinessDataApiTrustpilotSearchTaskPost::default()
        }
    }
}

/// Task request for the Trustpilot Reviews `task_post` endpoint.
/// See <https://docs.dataforseo.com/v3/business_data/trustpilot/reviews/task_post/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTrustpilotReviewsTaskPost {
    /// Business domain whose Trustpilot reviews to collect.
    pub domain: String,
    /// Number of reviews to parse (default `20`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<i32>,
    /// Review ordering, e.g. `"recency"` or `"relevance"`.
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

impl BusinessDataApiTrustpilotReviewsTaskPost {
    /// Builds a Trustpilot reviews task request for the given business domain.
    pub fn new<T: Into<String>>(domain: T) -> Self {
        BusinessDataApiTrustpilotReviewsTaskPost {
            domain: domain.into(),
            ..BusinessDataApiTrustpilotReviewsTaskPost::default()
        }
    }
}

/// Result of a Trustpilot Search request: metadata plus matched businesses.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTrustpilotSearchResult {
    /// Business name the task was set for.
    pub keyword: Option<String>,
    /// Result type, e.g. `"trustpilot_search"`.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Source domain the data was collected from (e.g. `"trustpilot.com"`).
    pub se_domain: Option<String>,
    /// URL of the Trustpilot results page the data was parsed from.
    pub check_url: Option<String>,
    /// UTC time the result was collected.
    pub datetime: Option<String>,
    /// Number of businesses in `items`.
    pub items_count: Option<i64>,
    /// Parsed business profiles.
    pub items: Option<Vec<BusinessDataApiTrustpilotSearchItem>>,
}

impl BusinessDataApiTrustpilotSearchResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiTrustpilotSearchItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// A single business profile returned by Trustpilot Search.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTrustpilotSearchItem {
    /// Element type, e.g. `"trustpilot_search_organic"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank within elements of the same type.
    pub rank_group: Option<i64>,
    /// Rank across all elements regardless of type.
    pub rank_absolute: Option<i64>,
    /// Business name.
    pub title: Option<String>,
    /// Business website domain.
    pub domain: Option<String>,
    /// URL of the business's Trustpilot profile.
    pub url: Option<String>,
    /// Business description.
    pub description: Option<String>,
    /// Business category on Trustpilot.
    pub category: Option<String>,
    /// Total number of reviews the business has received.
    pub reviews_count: Option<i64>,
    /// Aggregated TrustScore rating.
    pub rating: Option<BusinessDataApiRating>,
    /// Trustpilot verification status of the profile (raw object).
    pub verification: Option<Value>,
    /// Business contact details (raw object).
    pub contact_info: Option<Value>,
}

/// Result of a Trustpilot Reviews request: metadata plus individual reviews.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTrustpilotReviewsResult {
    /// Business domain the task was set for.
    pub domain: Option<String>,
    /// Result type, e.g. `"trustpilot_reviews"`.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Source domain the data was collected from (e.g. `"trustpilot.com"`).
    pub se_domain: Option<String>,
    /// URL of the Trustpilot business page the data was parsed from.
    pub check_url: Option<String>,
    /// UTC time the result was collected.
    pub datetime: Option<String>,
    /// Business name the reviews belong to.
    pub title: Option<String>,
    /// Business address shown on the profile.
    pub location: Option<String>,
    /// Total number of reviews the business has received.
    pub reviews_count: Option<i64>,
    /// Aggregated TrustScore rating.
    pub rating: Option<BusinessDataApiRating>,
    /// Number of reviews in `items`.
    pub items_count: Option<i64>,
    /// Parsed individual reviews.
    pub items: Option<Vec<BusinessDataApiTrustpilotReviewItem>>,
}

impl BusinessDataApiTrustpilotReviewsResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiTrustpilotReviewItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// A single Trustpilot review.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiTrustpilotReviewItem {
    /// Element type, e.g. `"trustpilot_review_search"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank within elements of the same type.
    pub rank_group: Option<i64>,
    /// Rank across all elements regardless of type.
    pub rank_absolute: Option<i64>,
    /// Placement of the element on the page.
    pub position: Option<String>,
    /// Direct URL to the review.
    pub url: Option<String>,
    /// Rating given by this review.
    pub rating: Option<BusinessDataApiRating>,
    /// Whether the review is from a verified purchase.
    pub verified: Option<bool>,
    /// Language code of the review.
    pub language: Option<String>,
    /// UTC timestamp the review was published.
    pub timestamp: Option<String>,
    /// Date of the experience the review describes.
    pub time_of_experience: Option<String>,
    /// Review headline.
    pub title: Option<String>,
    /// Review body text.
    pub review_text: Option<String>,
    /// Images attached to the review (raw objects).
    pub review_images: Option<Vec<Value>>,
    /// Reviewer profile details (raw object).
    pub user_profile: Option<Value>,
    /// Owner replies to the review (raw objects).
    pub responses: Option<Vec<Value>>,
    /// Number of likes the review received.
    pub likes_count: Option<i64>,
}
