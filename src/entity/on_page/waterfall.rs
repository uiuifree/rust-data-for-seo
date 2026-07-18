use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};

/// Result object for the OnPage Waterfall endpoint.
/// See <https://docs.dataforseo.com/v3/on_page/waterfall/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiWaterfall {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Number of items returned in this response.
    pub items_count: Option<i64>,
    /// Waterfall timing entries, one per requested page.
    pub items: Option<Vec<OnPageDataApiWaterfallItem>>,
}

/// Load-timing breakdown for a single page.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiWaterfallItem {
    /// Absolute URL of the analyzed page.
    pub page_url: Option<String>,
    /// Time until the page became interactive, in milliseconds.
    pub time_to_interactive: Option<i64>,
    /// Time until the DOM finished loading, in milliseconds.
    pub dom_complete: Option<i64>,
    /// Time to establish the connection, in milliseconds.
    pub connection_time: Option<i64>,
    /// Time to establish a secure (TLS) connection, in milliseconds.
    pub time_to_secure_connection: Option<i64>,
    /// Time until the request was sent, in milliseconds.
    pub request_sent_time: Option<i64>,
    /// Time spent waiting for the first response byte, in milliseconds.
    pub waiting_time: Option<i64>,
    /// Time spent downloading the response, in milliseconds.
    pub download_time: Option<i64>,
    /// Total time to load the page, in milliseconds.
    pub duration_time: Option<i64>,
    /// Offset from navigation start when fetching began, in milliseconds.
    pub fetch_start: Option<i64>,
    /// Offset from navigation start when fetching ended, in milliseconds.
    pub fetch_end: Option<i64>,
    /// Resources loaded by the page, each with its own timings.
    pub resources: Option<Vec<OnPageDataApiWaterfallItemResource>>,
}

/// Load-timing breakdown for a single resource of a page.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiWaterfallItemResource {
    /// Type of the resource, e.g. "script", "stylesheet", "image".
    pub resource_type: Option<String>,
    /// Absolute URL of the resource.
    pub url: Option<String>,
    /// URL or entity that initiated the request for the resource.
    pub initiator: Option<String>,
    /// Total time to load the resource, in milliseconds.
    pub duration_time: Option<i64>,
    /// Offset from page navigation start when fetching began, in milliseconds.
    pub fetch_start: Option<i64>,
    /// Offset from page navigation start when fetching ended, in milliseconds.
    pub fetch_end: Option<i64>,
    /// Position in the source where the resource is referenced.
    pub location: Option<OnPageDataApiWaterfallItemResourceLocation>,
    /// Whether the resource blocks rendering of the page.
    pub is_render_blocking: Option<bool>,
}

/// Location of a resource reference within the page source.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiWaterfallItemResourceLocation {
    /// Line number in the source where the resource is referenced.
    pub line: Option<i32>,
    /// Horizontal offset of the reference, in pixels.
    pub offset_left: Option<i32>,
    /// Vertical offset of the reference, in pixels.
    pub offset_top: Option<i32>,
}
