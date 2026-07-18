use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};

/// Crawled pages that cannot be indexed by search engines.
/// See <https://docs.dataforseo.com/v3/on_page/non_indexable/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiNonIndexable {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Total number of matching items in the database.
    pub total_items_count: Option<i64>,
    /// Number of items returned in this response.
    pub items_count: Option<i64>,
    /// Non-indexable page items returned in this response.
    pub items: Option<Vec<OnPageDataApiNonIndexableItem>>,
}

/// A single non-indexable page and why it cannot be indexed.
/// See <https://docs.dataforseo.com/v3/on_page/non_indexable/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiNonIndexableItem {
    /// Reason the page is non-indexable (e.g. "noindex meta tag").
    pub reason: Option<String>,
    /// URL of the non-indexable page.
    pub url: Option<String>,
}
