use crate::entity::{OnPageDataApiCrawlStatus, OnPageResourceHtml};
use serde::{Deserialize, Serialize};

/// Pages with duplicate content detected across the crawled site.
/// See <https://docs.dataforseo.com/v3/on_page/duplicate_content/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiDuplicateContent {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Number of items returned in this response.
    pub items_count: Option<i64>,
    /// Duplicate-content groups returned in this response.
    pub items: Option<Vec<OnPageDataApiDuplicateContentItem>>,
}

/// A page and the other pages whose content duplicates it.
/// See <https://docs.dataforseo.com/v3/on_page/duplicate_content/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiDuplicateContentItem {
    /// URL of the page compared against.
    pub url: Option<String>,
    /// Number of pages that duplicate this page's content.
    pub total_count: Option<i64>,
    /// Pages that share duplicate content with this page.
    pub pages: Option<Vec<OnPageDataApiDuplicateContentPage>>,
}

/// A page duplicating another, with its similarity score.
/// See <https://docs.dataforseo.com/v3/on_page/duplicate_content/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiDuplicateContentPage {
    /// Content similarity to the compared page as a fraction (0.0-1.0).
    pub similarity: Option<f64>,
    /// Full page resource data for the duplicating page.
    pub page: Option<OnPageResourceHtml>,
}
