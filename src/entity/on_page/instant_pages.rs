use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OnPage Instant Pages result: on-the-fly page analysis without a prior task.
/// See <https://docs.dataforseo.com/v3/on_page/instant_pages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiInstantPages {
    /// Status of the crawling session (`in_progress`, `finished`).
    pub crawl_progress: Option<String>,
    /// Details of the crawling session; `null` for instant pages.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// IP address the crawler used to initiate the session.
    pub crawl_gateway_address: Option<String>,
    /// Total number of relevant items in the database.
    pub total_items_count: Option<i64>,
    /// Number of items in the `items` array.
    pub items_count: Option<i32>,
    /// Array of analyzed pages.
    pub items: Option<Vec<OnPageDataApiInstantPagesItem>>,
}

/// A single analyzed page in an Instant Pages result.
/// See <https://docs.dataforseo.com/v3/on_page/instant_pages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiInstantPagesItem {
    /// Type of returned resource (`html`, `broken`, `redirect`, `script`, `image`, `stylesheet`).
    pub resource_type: Option<String>,
    /// HTTP response code of the page.
    pub status_code: Option<i32>,
    /// Location header value; redirect target URL.
    pub location: Option<String>,
    /// URL of the analyzed page.
    pub url: Option<String>,
    /// Meta information extracted from the page.
    pub meta: Option<OnPageDataApiInstantPagesMeta>,
    /// Page load timing metrics.
    pub page_timing: Option<OnPageDataApiInstantPagesPageTiming>,
    /// Page optimization score on a 100-point scale.
    pub onpage_score: Option<f64>,
    /// Total DOM (Document Object Model) size of the page.
    pub total_dom_size: Option<i64>,
    /// Result of executing the specified custom JavaScript; shape depends on the script.
    pub custom_js_response: Option<Value>,
    /// Error message if custom JavaScript execution failed.
    pub custom_js_client_exception: Option<String>,
    /// Resource loading errors and warnings for the page.
    pub resource_errors: Option<OnPageDataApiInstantPagesResourceErrors>,
    /// Whether the page contains broken resources.
    pub broken_resources: Option<bool>,
    /// Whether the page contains broken links.
    pub broken_links: Option<bool>,
    /// Whether a duplicate title tag exists.
    pub duplicate_title: Option<bool>,
    /// Whether a duplicate description exists.
    pub duplicate_description: Option<bool>,
    /// Whether duplicate content exists.
    pub duplicate_content: Option<bool>,
    /// Number of clicks from the homepage to reach this page.
    pub click_depth: Option<i32>,
    /// Resource size, in bytes.
    pub size: Option<i64>,
    /// Page size after encoding, in bytes.
    pub encoded_size: Option<i64>,
    /// Compressed page size, in bytes.
    pub total_transfer_size: Option<i64>,
    /// Date and time when the resource was fetched, in UTC (`yyyy-mm-dd hh:mm:ss +00:00`).
    pub fetch_time: Option<String>,
    /// Cache-control directives for the page.
    pub cache_control: Option<OnPageDataApiInstantPagesCacheControl>,
    /// On-page check results keyed by check name.
    pub checks: Option<Value>,
    /// Compression algorithm applied to the content.
    pub content_encoding: Option<String>,
    /// Media type used to display the page (e.g. `text/html`).
    pub media_type: Option<String>,
    /// Server version information.
    pub server: Option<String>,
    /// Whether the page is a single resource.
    pub is_resource: Option<bool>,
    /// Page URL length, in characters.
    pub url_length: Option<i32>,
    /// Relative URL length, in characters.
    pub relative_url_length: Option<i32>,
    /// Last-modification dates reported for the page.
    pub last_modified: Option<OnPageDataApiInstantPagesLastModified>,
}

/// Meta information extracted from an analyzed page.
/// See <https://docs.dataforseo.com/v3/on_page/instant_pages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiInstantPagesMeta {
    /// Page title.
    pub title: Option<String>,
    /// Code page of the document (e.g. `65001` for UTF-8).
    pub charset: Option<i32>,
    /// Whether meta robots allow crawlers to follow the page links.
    pub follow: Option<bool>,
    /// Value of the meta generator tag.
    pub generator: Option<String>,
    /// HTML header tags (h1–h6) found on the page.
    pub htags: Option<Value>,
    /// Content of the meta description tag.
    pub description: Option<String>,
    /// Page favicon URL.
    pub favicon: Option<String>,
    /// Content of the meta keywords tag.
    pub meta_keywords: Option<String>,
    /// Canonical page URL.
    pub canonical: Option<String>,
    /// Number of internal links on the page.
    pub internal_links_count: Option<i32>,
    /// Number of external links on the page.
    pub external_links_count: Option<i32>,
    /// Number of internal links pointing to the page.
    pub inbound_links_count: Option<i32>,
    /// Number of images on the page.
    pub images_count: Option<i32>,
    /// Total size of images on the page, in bytes.
    pub images_size: Option<i64>,
    /// Number of scripts on the page.
    pub scripts_count: Option<i32>,
    /// Total size of scripts on the page, in bytes.
    pub scripts_size: Option<i64>,
    /// Number of stylesheets on the page.
    pub stylesheets_count: Option<i32>,
    /// Total size of stylesheets on the page, in bytes.
    pub stylesheets_size: Option<i64>,
    /// Title tag length, in characters.
    pub title_length: Option<i32>,
    /// Description tag length, in characters.
    pub description_length: Option<i32>,
    /// Number of scripts that block page rendering.
    pub render_blocking_scripts_count: Option<i32>,
    /// Number of stylesheets that block page rendering.
    pub render_blocking_stylesheets_count: Option<i32>,
    /// Cumulative Layout Shift (CLS) Core Web Vitals metric.
    pub cumulative_layout_shift: Option<f64>,
    /// Content of the meta title tag in the head section.
    pub meta_title: Option<String>,
    /// Readability and content-consistency metrics for the page.
    pub content: Option<OnPageDataApiInstantPagesContent>,
    /// Deprecated HTML tags found on the page.
    pub deprecated_tags: Option<Vec<String>>,
    /// Duplicate meta tags found on the page.
    pub duplicate_meta_tags: Option<Vec<String>>,
    /// Spellcheck results (Hunspell-based).
    pub spell: Option<Value>,
    /// Open Graph and Twitter card tags found on the page.
    pub social_media_tags: Option<Value>,
}

/// Readability and content-consistency metrics for an analyzed page.
/// See <https://docs.dataforseo.com/v3/on_page/instant_pages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiInstantPagesContent {
    /// Total size of plain text on the page, in bytes.
    pub plain_text_size: Option<i64>,
    /// Ratio of `plain_text_size` to overall page `size`.
    pub plain_text_rate: Option<f64>,
    /// Number of words in the page text.
    pub plain_text_word_count: Option<i32>,
    /// Automated Readability Index score for the page text.
    pub automated_readability_index: Option<f64>,
    /// Coleman-Liau readability index for the page text.
    pub coleman_liau_readability_index: Option<f64>,
    /// Dale-Chall readability index for the page text.
    pub dale_chall_readability_index: Option<f64>,
    /// Flesch-Kincaid readability index for the page text.
    pub flesch_kincaid_readability_index: Option<f64>,
    /// SMOG readability index for the page text.
    pub smog_readability_index: Option<f64>,
    /// Consistency of the meta description with page content (0 to 1).
    pub description_to_content_consistency: Option<f64>,
    /// Consistency of the meta title with page content (0 to 1).
    pub title_to_content_consistency: Option<f64>,
    /// Consistency of the meta keywords with page content (0 to 1).
    pub meta_keywords_to_content_consistency: Option<f64>,
}

/// Page load timing metrics, in milliseconds.
/// See <https://docs.dataforseo.com/v3/on_page/instant_pages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiInstantPagesPageTiming {
    /// Time until the user can interact with the page, in milliseconds.
    pub time_to_interactive: Option<i32>,
    /// Time until the page and its subresources finish loading, in milliseconds.
    pub dom_complete: Option<i32>,
    /// Largest Contentful Paint Core Web Vitals metric, in milliseconds.
    pub largest_contentful_paint: Option<f64>,
    /// First Input Delay Core Web Vitals metric, in milliseconds.
    pub first_input_delay: Option<f64>,
    /// Time to establish a connection to the server, in milliseconds.
    pub connection_time: Option<i32>,
    /// Time to establish a secure connection, in milliseconds.
    pub time_to_secure_connection: Option<i32>,
    /// Time to send the request to the server, in milliseconds.
    pub request_sent_time: Option<i32>,
    /// Time to first byte (TTFB), in milliseconds.
    pub waiting_time: Option<i32>,
    /// Time for the browser to receive the response, in milliseconds.
    pub download_time: Option<i32>,
    /// Total time for the complete response from the server, in milliseconds.
    pub duration_time: Option<i32>,
    /// Time to start downloading the HTML resource, in milliseconds.
    pub fetch_start: Option<i32>,
    /// Time to finish downloading the HTML resource, in milliseconds.
    pub fetch_end: Option<i32>,
}

/// Cache-control directives for an analyzed page.
/// See <https://docs.dataforseo.com/v3/on_page/instant_pages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiInstantPagesCacheControl {
    /// Whether the page or resource is cacheable.
    pub cachable: Option<bool>,
    /// Browser cache lifetime (time to live), in milliseconds.
    pub ttl: Option<i64>,
}

/// Resource loading errors and warnings for an analyzed page.
/// See <https://docs.dataforseo.com/v3/on_page/instant_pages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiInstantPagesResourceErrors {
    /// Parsing errors, each with line, column, message, and status code.
    pub errors: Option<Vec<Value>>,
    /// Warnings (e.g. node nesting, DOM size, HTML depth) with line, column, message, and status code.
    pub warnings: Option<Vec<Value>>,
}

/// Last-modification dates reported for an analyzed page, in UTC.
/// See <https://docs.dataforseo.com/v3/on_page/instant_pages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiInstantPagesLastModified {
    /// Last-modification date from the HTTP header; `null` if unavailable.
    pub header: Option<String>,
    /// Last-modification date from the sitemap; `null` if unavailable.
    pub sitemap: Option<String>,
    /// Last-modification date from a meta tag; `null` if unavailable.
    pub meta_tag: Option<String>,
}
