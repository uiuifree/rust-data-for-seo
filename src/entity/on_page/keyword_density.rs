use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};

/// Keyword density statistics for the crawled pages.
/// See <https://docs.dataforseo.com/v3/on_page/keyword_density/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiKeywordDensity {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Number of items returned in this response.
    pub items_count: Option<i64>,
    /// Keyword density items returned in this response.
    pub items: Option<Vec<OnPageDataApiKeywordDensityItem>>,
}

/// Density of a single keyword or phrase on the page.
/// See <https://docs.dataforseo.com/v3/on_page/keyword_density/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiKeywordDensityItem {
    /// The keyword or phrase measured.
    pub keyword: Option<String>,
    /// Number of times the keyword appears on the page.
    pub frequency: Option<i64>,
    /// Keyword density as a fraction of total words (0.0-1.0).
    pub density: Option<f64>,
}
