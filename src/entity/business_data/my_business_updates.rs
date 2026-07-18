//! Entities for Google My Business Updates (`task_get`).
//! See <https://docs.dataforseo.com/v3/business_data/google/my_business_updates/task_get/>.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of a My Business Updates request: metadata plus posted updates.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiMyBusinessUpdatesResult {
    /// Business name or identifier the task was set for.
    pub keyword: Option<String>,
    /// Result type, e.g. `"my_business_updates"`.
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
    /// Google-defined customer identifier (CID) for the business.
    pub cid: Option<String>,
    /// Unique Google feature identifier for the business.
    pub feature_id: Option<String>,
    /// Distinct element types present in `items`.
    pub item_types: Option<Vec<String>>,
    /// Number of elements in `items`.
    pub items_count: Option<i64>,
    /// Parsed business post elements.
    pub items: Option<Vec<BusinessDataApiMyBusinessUpdateItem>>,
}

impl BusinessDataApiMyBusinessUpdatesResult {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[BusinessDataApiMyBusinessUpdateItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// A single business post published on a Google Business Profile.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiMyBusinessUpdateItem {
    /// Element type, e.g. `"business_update"`.
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
    /// Text content of the post.
    pub snippet: Option<String>,
    /// URL of the post's primary image.
    pub image_url: Option<String>,
    /// Human-readable date the post was published.
    pub post_date: Option<String>,
    /// UTC timestamp the post was published.
    pub timestamp: Option<String>,
    /// Links embedded in the post (raw objects).
    pub links: Option<Vec<Value>>,
    /// Images attached to the post (raw objects).
    pub images: Option<Vec<Value>>,
    /// Kind of update (e.g. offer, event, product).
    pub update_type: Option<String>,
    /// Google-defined customer identifier (CID) for the business.
    pub cid: Option<String>,
    /// Unique identifier of the post.
    pub post_id: Option<String>,
    /// Direct URL to the post.
    pub url: Option<String>,
}
