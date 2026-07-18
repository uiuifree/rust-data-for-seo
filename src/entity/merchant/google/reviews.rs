use crate::entity::{MerchantApiRating, MerchantApiSpell};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of a Google Shopping reviews task (`task_get/advanced`).
/// See <https://docs.dataforseo.com/v3/merchant/google/reviews/task_get/advanced/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleReviews {
    /// Google Shopping product identifier the reviews were requested for.
    pub product_id: Option<String>,
    /// Result type identifier returned by the API.
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    /// Google domain the lookup was run on (e.g. `google.com`).
    pub se_domain: Option<String>,
    /// Location code the lookup was localized to.
    pub location_code: Option<i32>,
    /// Language code the lookup was localized to (e.g. `en`).
    pub language_code: Option<String>,
    /// Direct URL to the Google Shopping reviews page.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was retrieved.
    pub datetime: Option<String>,
    /// Spelling autocorrection applied to the query, if any.
    pub spell: Option<MerchantApiSpell>,
    /// Title of the product the reviews belong to.
    pub title: Option<String>,
    /// URL of the product image.
    pub image_url: Option<String>,
    /// Aggregated product rating.
    pub rating: Option<MerchantApiRating>,
    /// Distribution of reviews across rating buckets.
    pub rating_groups: Option<Vec<Value>>,
    /// Keywords most frequently mentioned in the reviews.
    pub top_keywords: Option<Vec<Value>>,
    /// Total number of reviews for the product.
    pub reviews_count: Option<i64>,
    /// Item types present in the `items` array.
    pub item_types: Option<Vec<String>>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Parsed review elements.
    pub items: Option<Vec<MerchantApiGoogleReviewItem>>,
}

/// A single review element of a Google Shopping reviews result.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MerchantApiGoogleReviewItem {
    /// Element type identifier.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Rank of the review within its own `item_type`.
    pub rank_group: Option<i64>,
    /// Absolute rank of the review across all items.
    pub rank_absolute: Option<i64>,
    /// Position of the review on the page (e.g. `left`).
    pub position: Option<String>,
    /// URLs of images attached to the review.
    pub images: Option<Vec<String>>,
    /// Title / headline of the review.
    pub title: Option<String>,
    /// URL of the review source.
    pub url: Option<String>,
    /// Full text of the review.
    pub review_text: Option<String>,
    /// Source that provided the review (e.g. the merchant name).
    pub provided_by: Option<String>,
    /// Name of the review author.
    pub author: Option<String>,
    /// Publication date of the review.
    pub publication_date: Option<String>,
    /// Rating given in the review.
    pub rating: Option<MerchantApiRating>,
}
