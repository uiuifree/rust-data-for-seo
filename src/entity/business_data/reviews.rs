//! Entities for Google Reviews and Extended Reviews (`task_get`).
//! See <https://docs.dataforseo.com/v3/business_data/google/reviews/task_get/>.

use crate::entity::BusinessDataApiRating;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of a Reviews request: business metadata plus individual reviews.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiReviewsResult {
    /// Business name or identifier the task was set for.
    pub keyword: Option<String>,
    /// Result type, e.g. `"reviews"`.
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
    /// Business name the reviews belong to.
    pub title: Option<String>,
    /// Secondary title or category shown under the business name.
    pub sub_title: Option<String>,
    /// Unique Google feature identifier for the business.
    pub feature_id: Option<String>,
    /// Google Place identifier for the business.
    pub place_id: Option<String>,
    /// Google-defined customer identifier (CID) for the business.
    pub cid: Option<String>,
    /// Total number of reviews the business has received.
    pub reviews_count: Option<i64>,
    /// Aggregated rating of the business.
    pub rating: Option<BusinessDataApiRating>,
    /// Number of reviews in `items`.
    pub items_count: Option<i64>,
    /// Parsed individual reviews.
    pub items: Option<Vec<BusinessDataApiReviewItem>>,
}

impl BusinessDataApiReviewsResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiReviewItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// A single Google review.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiReviewItem {
    /// Element type, e.g. `"google_review"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank within elements of the same type.
    pub rank_group: Option<i64>,
    /// Rank across all elements regardless of type.
    pub rank_absolute: Option<i64>,
    /// Placement of the element on the page.
    pub position: Option<String>,
    /// XPath of the element in the source page.
    pub xpath: Option<String>,
    /// Review body text.
    pub review_text: Option<String>,
    /// Original, untranslated review text.
    pub original_review_text: Option<String>,
    /// Language code of the original review text.
    pub original_language: Option<String>,
    /// Human-readable time since the review was posted.
    pub time_ago: Option<String>,
    /// UTC timestamp the review was posted.
    pub timestamp: Option<String>,
    /// Rating given by this review.
    pub rating: Option<BusinessDataApiRating>,
    /// Number of reviews written by the reviewer.
    pub reviews_count: Option<i64>,
    /// Number of photos attached to the review.
    pub photos_count: Option<i64>,
    /// Whether the reviewer is a Google Local Guide.
    pub local_guide: Option<bool>,
    /// Display name of the reviewer.
    pub profile_name: Option<String>,
    /// URL of the reviewer's Google profile.
    pub profile_url: Option<String>,
    /// Direct URL to the review.
    pub review_url: Option<String>,
    /// URL of the reviewer's profile image.
    pub profile_image_url: Option<String>,
    /// Owner's reply to the review, if any.
    pub owner_answer: Option<String>,
    /// Original, untranslated owner reply.
    pub original_owner_answer: Option<String>,
    /// Human-readable time since the owner replied.
    pub owner_time_ago: Option<String>,
    /// UTC timestamp the owner replied.
    pub owner_timestamp: Option<String>,
    /// Unique identifier of the review.
    pub review_id: Option<String>,
    /// Images attached to the review (raw objects).
    pub images: Option<Vec<Value>>,
    /// Highlighted snippets from the review (raw objects).
    pub review_highlights: Option<Vec<Value>>,
}
