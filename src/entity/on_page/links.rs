use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};

/// Links found between the crawled pages of the target website.
/// See <https://docs.dataforseo.com/v3/on_page/links/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiLinks {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Total number of matching items in the database.
    pub total_items_count: Option<i64>,
    /// Number of items returned in this response.
    pub items_count: Option<i64>,
    /// Link items returned in this response.
    pub items: Option<Vec<OnPageDataApiLinkItem>>,
}

/// A single link discovered on a crawled page.
/// See <https://docs.dataforseo.com/v3/on_page/links/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiLinkItem {
    /// Link classification: "anchor", "image", "canonical", "alternate", etc.
    #[serde(rename = "type")]
    pub link_type: Option<String>,
    /// Referring domain where the link originates.
    pub domain_from: Option<String>,
    /// Referenced domain the link points to.
    pub domain_to: Option<String>,
    /// Relative URL of the page on which the link was found.
    pub page_from: Option<String>,
    /// Relative URL of the page the link points to.
    pub page_to: Option<String>,
    /// Absolute URL of the page on which the link was found.
    pub link_from: Option<String>,
    /// Absolute URL of the page the link points to.
    pub link_to: Option<String>,
    /// True when the link is not marked nofollow.
    pub dofollow: Option<bool>,
    /// URL scheme of the referring page: "http" or "https".
    pub page_from_scheme: Option<String>,
    /// URL scheme of the referenced page: "http" or "https".
    pub page_to_scheme: Option<String>,
    /// Link scope: "internal" or "external".
    pub direction: Option<String>,
    /// True when the link points to a broken or unreachable resource.
    pub is_broken: Option<bool>,
    /// True when nofollow and dofollow links to the same target conflict.
    pub is_link_relation_conflict: Option<bool>,
    /// HTTP status code returned by the destination page.
    pub page_to_status_code: Option<i32>,
    /// Anchor text of the link.
    pub text: Option<String>,
    /// Alternative text of the linked image.
    pub image_alt: Option<String>,
    /// Source URL of the linked image.
    pub image_src: Option<String>,
    /// Value of the link's rel attribute (e.g. "ugc", "noopener").
    pub link_attribute: Option<String>,
    /// True when the hreflang implementation is valid.
    pub is_valid_hreflang: Option<bool>,
    /// Language and country code declared by the hreflang attribute.
    pub hreflang: Option<String>,
}
