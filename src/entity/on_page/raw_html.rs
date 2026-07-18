use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};

/// Raw HTML of a crawled page.
/// See <https://docs.dataforseo.com/v3/on_page/raw_html/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiRawHtml {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Number of items returned in this response.
    pub items_count: Option<i32>,
    /// Raw HTML item returned in this response.
    pub items: Option<OnPageDataApiRawHtmlItem>,
}

/// Container for the raw HTML source of a page.
/// See <https://docs.dataforseo.com/v3/on_page/raw_html/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiRawHtmlItem {
    /// Raw HTML source of the page.
    pub html: Option<String>,
}
