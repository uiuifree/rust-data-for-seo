use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result object for the OnPage Resources endpoint.
/// See <https://docs.dataforseo.com/v3/on_page/resources/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiResources {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// Total number of matching items in the database.
    pub total_items_count: Option<i64>,
    /// Number of items returned in this response.
    pub items_count: Option<i64>,
    /// Resources found during the crawl.
    pub items: Option<Vec<OnPageDataApiResourceItem>>,
}

/// A single resource discovered during the crawl.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiResourceItem {
    /// Type of the resource, e.g. "html", "script", "stylesheet", "image".
    pub resource_type: Option<String>,
    /// Meta information about the resource.
    pub meta: Option<OnPageResourceItemMeta>,
    /// HTTP status code returned for the resource.
    pub status_code: Option<i32>,
    /// Redirect target URL, if the resource redirects.
    pub location: Option<String>,
    /// Absolute URL of the resource.
    pub url: Option<String>,
    /// Uncompressed size of the resource, in bytes.
    pub size: Option<i64>,
    /// Encoded (compressed) size of the resource, in bytes.
    pub encoded_size: Option<i64>,
    /// Total bytes transferred over the network, including headers.
    pub total_transfer_size: Option<i64>,
    /// Timestamp when the resource was fetched (UTC).
    pub fetch_time: Option<String>,
    /// Breakdown of the fetch timing for the resource.
    pub fetch_timing: Option<OnPageResourceItemFetchTiming>,
    /// Cache-control settings for the resource.
    pub cache_control: Option<OnPageResourceItemCacheControl>,
    /// Boolean flags for on-page checks performed on the resource.
    pub checks: Option<Value>,
    /// Content encoding applied to the resource, e.g. "gzip".
    pub content_encoding: Option<String>,
    /// Media (MIME) type of the resource.
    pub media_type: Option<String>,
    /// Category of the resource by accept type, e.g. "text_html".
    pub accept_type: Option<String>,
    /// Value of the `Server` response header.
    pub server: Option<String>,
    /// Last-modified dates reported for the resource.
    pub last_modified: Option<OnPageResourceItemLastModified>,
    /// Errors and warnings found while parsing the resource.
    pub resource_errors: Option<OnPageResourceItemErrors>,
}

/// Meta information about a resource, such as image attributes.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageResourceItemMeta {
    /// Alt text of the image resource.
    pub alternative_text: Option<String>,
    /// Title attribute of the resource.
    pub title: Option<String>,
    /// Original width of the image, in pixels.
    pub original_width: Option<i32>,
    /// Original height of the image, in pixels.
    pub original_height: Option<i32>,
    /// Rendered width of the image, in pixels.
    pub width: Option<i32>,
    /// Rendered height of the image, in pixels.
    pub height: Option<i32>,
}

/// Timing breakdown for fetching a resource.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageResourceItemFetchTiming {
    /// Total time to fetch the resource, in milliseconds.
    pub duration_time: Option<i32>,
    /// Offset from crawl start when fetching began, in milliseconds.
    pub fetch_start: Option<i32>,
    /// Offset from crawl start when fetching ended, in milliseconds.
    pub fetch_end: Option<i32>,
}

/// Cache-control settings reported for a resource.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageResourceItemCacheControl {
    /// Whether the resource can be cached.
    pub cachable: Option<bool>,
    /// Time the resource stays valid in cache, in seconds.
    pub ttl: Option<i32>,
}

/// Last-modified dates reported for a resource from various sources.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageResourceItemLastModified {
    /// Last-modified date from the HTTP response header (UTC).
    pub header: Option<String>,
    /// Last-modified date declared in the sitemap (UTC).
    pub sitemap: Option<String>,
    /// Last-modified date declared in a meta tag (UTC).
    pub meta_tag: Option<String>,
}

/// Errors and warnings collected while parsing a resource.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageResourceItemErrors {
    /// Errors encountered while parsing the resource.
    pub errors: Option<Vec<OnPageResourceItemError>>,
    /// Warnings encountered while parsing the resource.
    pub warnings: Option<Vec<OnPageResourceItemWarning>>,
}

/// A single parsing error found in a resource.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageResourceItemError {
    /// Line number where the error occurred.
    pub line: Option<i32>,
    /// Column number where the error occurred.
    pub column: Option<i32>,
    /// Human-readable description of the error.
    pub message: Option<String>,
    /// Status code associated with the error.
    pub status_code: Option<i32>,
}

/// A single parsing warning found in a resource.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageResourceItemWarning {
    /// Line number where the warning occurred.
    pub line: Option<i32>,
    /// Column number where the warning occurred.
    pub column: Option<i32>,
    /// Human-readable description of the warning.
    pub message: Option<String>,
    /// Status code associated with the warning.
    pub status_code: Option<i32>,
}
