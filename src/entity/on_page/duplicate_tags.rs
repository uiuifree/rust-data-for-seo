use crate::entity::{OnPageDataApiCrawlStatus, OnPageResourceHtml};
use serde::{Deserialize, Serialize};

/// Pages that share duplicate title, description or heading tags.
/// See <https://docs.dataforseo.com/v3/on_page/duplicate_tags/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiDuplicateTags {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Total number of pages with duplicate tags in the database.
    pub total_pages_count: Option<i64>,
    /// Number of pages returned in this response.
    pub pages_count: Option<i64>,
    /// Number of items returned in this response.
    pub items_count: Option<i64>,
    /// Duplicate-tag groups returned in this response.
    pub items: Option<Vec<OnPageDataApiDuplicateTagItem>>,
}

/// A duplicated tag value and the pages that share it.
/// See <https://docs.dataforseo.com/v3/on_page/duplicate_tags/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiDuplicateTagItem {
    /// The duplicated tag value shared by the pages.
    pub accumulator: Option<String>,
    /// Number of pages that share this tag value.
    pub total_count: Option<i64>,
    /// Pages that share the duplicated tag value.
    pub pages: Option<Vec<OnPageResourceHtml>>,
}
