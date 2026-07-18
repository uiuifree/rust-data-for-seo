use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};

/// Screenshot of a crawled page rendered by the OnPage API.
/// See <https://docs.dataforseo.com/v3/on_page/page_screenshot/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiPageScreensShot {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Error message returned if the screenshot could not be captured.
    pub error_message: Option<String>,
    /// Number of items returned in this response.
    pub items_count: Option<i32>,
    /// Screenshot items returned in this response.
    pub items: Option<Vec<OnPageDataApiPageScreensShotItem>>,
}

/// A single captured page screenshot.
/// See <https://docs.dataforseo.com/v3/on_page/page_screenshot/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiPageScreensShotItem {
    /// Screenshot image encoded as a Base64 string.
    pub image: Option<String>,
}
