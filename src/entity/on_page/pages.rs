use crate::entity::{OnPageDataApiCrawlStatus, OnPageResourceHtml};
use serde::{Deserialize, Serialize};

/// Crawled pages of the target website with their on-page data.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiPage {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Total number of matching items in the database.
    pub total_items_count: Option<i32>,
    /// Number of items returned in this response.
    pub items_count: Option<i32>,
    /// Page resource items returned in this response.
    pub items: Option<Vec<OnPageResourceHtml>>,
}
