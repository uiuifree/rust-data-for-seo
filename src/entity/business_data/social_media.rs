//! Entities for the Social Media endpoints (Pinterest).
//! See <https://docs.dataforseo.com/v3/business_data/social_media/overview/>.

use serde::{Deserialize, Serialize};

/// Request body for the Social Media `live` endpoints.
/// See <https://docs.dataforseo.com/v3/business_data/social_media/pinterest/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiSocialMediaLiveRequest {
    /// Target URLs to look up (up to 10; each must include `http://`/`https://`).
    pub targets: Vec<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl BusinessDataApiSocialMediaLiveRequest {
    /// Builds a request for the given target URLs (each must include a scheme).
    pub fn new(targets: Vec<String>) -> Self {
        BusinessDataApiSocialMediaLiveRequest { targets, tag: None }
    }
}

/// A single Pinterest engagement result for one target URL.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BusinessDataApiPinterestItem {
    /// Element type, e.g. `"social_media_pinterest_item"`.
    #[serde(rename = "type")]
    pub item_type: Option<String>,
    /// Target URL this result corresponds to.
    pub page_url: Option<String>,
    /// Number of times the URL has been pinned to Pinterest.
    pub pins_count: Option<i64>,
}
