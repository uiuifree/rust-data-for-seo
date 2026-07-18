use crate::entity::AppDataApiRating;
use serde::{Deserialize, Serialize};

/// Result item of an App Reviews `task_get/advanced` response.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_reviews/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiAppReviewsResult {
    /// Store identifier of the reviewed application.
    pub app_id: Option<String>,
    /// Result type identifying the app reviews lookup.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// App store domain the results were fetched from (e.g. `play.google.com`).
    pub se_domain: Option<String>,
    /// Location code the lookup was localized to.
    pub location_code: Option<i32>,
    /// Language code the lookup was localized to.
    pub language_code: Option<String>,
    /// Direct URL to the application's store page.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was received.
    pub datetime: Option<String>,
    /// Name of the reviewed application.
    pub title: Option<String>,
    /// Aggregated user rating of the application.
    pub rating: Option<AppDataApiRating>,
    /// Total number of reviews the application has received.
    pub reviews_count: Option<i64>,
    /// Types of result elements present in `items`.
    pub item_types: Option<Vec<String>>,
    /// Number of elements returned in `items`.
    pub items_count: Option<i32>,
    /// Reviews returned for the application.
    pub items: Option<Vec<AppDataApiAppReviewsItem>>,
}

/// A single review of an application.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_reviews/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiAppReviewsItem {
    /// Element type (e.g. `google_play_reviews_search`).
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Position among items of the same `type`.
    pub rank_group: Option<i32>,
    /// Absolute position among all returned items.
    pub rank_absolute: Option<i32>,
    /// Alignment of the element within the results (e.g. `left`).
    pub position: Option<String>,
    /// Store identifier of the review.
    pub id: Option<String>,
    /// Application version the review refers to.
    pub version: Option<String>,
    /// Review title, when the store provides one.
    pub title: Option<String>,
    /// Body text of the review.
    pub review_text: Option<String>,
    /// UTC timestamp when the review was published.
    pub timestamp: Option<String>,
    /// Number of users who marked the review as helpful.
    pub helpful_count: Option<i64>,
    /// Star rating given by the reviewer.
    pub rating: Option<AppDataApiRating>,
    /// Profile of the review's author.
    pub user_profile: Option<AppDataApiReviewUserProfile>,
    /// Developer replies to the review.
    pub responses: Option<Vec<AppDataApiReviewResponse>>,
}

/// Author profile attached to a review.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_reviews/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiReviewUserProfile {
    /// Display name of the reviewer.
    pub profile_name: Option<String>,
    /// URL of the reviewer's profile image.
    pub profile_image_url: Option<String>,
    /// URL of the reviewer's profile.
    pub profile_url: Option<String>,
}

/// A developer reply attached to a review.
/// See <https://docs.dataforseo.com/v3/app_data/google/app_reviews/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppDataApiReviewResponse {
    /// Name of the responding developer.
    pub author: Option<String>,
    /// Title of the response, when present.
    pub title: Option<String>,
    /// Body text of the developer response.
    pub text: Option<String>,
    /// UTC timestamp when the response was published.
    pub timestamp: Option<String>,
}
