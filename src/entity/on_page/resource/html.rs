use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A crawled HTML page returned by the OnPage Pages endpoint.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceHtml {
    /// Type of the returned resource (e.g. `html`, `broken`, `redirect`).
    pub resource_type: Option<String>,
    /// HTTP status code returned when fetching the page.
    pub status_code: Option<i32>,
    /// Location header value indicating the URL the page redirects to.
    pub location: Option<String>,
    /// URL of the analyzed page.
    pub url: Option<String>,
    /// Meta information extracted from the page.
    pub meta: Option<OnPageResourceMeta>,
    /// Page load timing metrics.
    pub page_timing: Option<OnPageResourceHtmlPageTiming>,
    /// Overall optimization score on a 100-point scale.
    pub onpage_score: Option<f64>,
    /// Total DOM size of the page, in element count.
    pub total_dom_size: Option<i64>,
    /// Response of the custom JavaScript executed on the page.
    pub custom_js_response: Option<Value>,
    /// Exception thrown by the custom client-side JavaScript, if any.
    pub custom_js_client_exception: Option<String>,
    /// True when the page contains broken resources.
    pub broken_resources: Option<bool>,
    /// True when the page contains broken links.
    pub broken_links: Option<bool>,
    /// True when the page has a duplicate title tag.
    pub duplicate_title: Option<bool>,
    /// True when the page has a duplicate meta description.
    pub duplicate_description: Option<bool>,
    /// True when the page has duplicate content.
    pub duplicate_content: Option<bool>,
    /// Number of clicks required to reach the page from the homepage.
    pub click_depth: Option<i32>,
    /// Uncompressed size of the page, in bytes.
    pub size: Option<i64>,
    /// Page size after encoding, in bytes.
    pub encoded_size: Option<i64>,
    /// Compressed size transferred over the network, in bytes.
    pub total_transfer_size: Option<i64>,
    /// Date and time when the resource was fetched (UTC).
    pub fetch_time: Option<String>,
    /// Cache-control information for the page.
    pub cache_control: Option<OnPageResourceHtmlCacheControl>,
    /// Result of the on-page SEO checks for the page.
    pub checks: Option<OnPageResourceHtmlWebsiteChecks>,
    /// Compression algorithm applied to the content.
    pub content_encoding: Option<String>,
    /// Media type used to display the page (e.g. `text/html`).
    pub media_type: Option<String>,
    /// Server version information from the response headers.
    pub server: Option<String>,
    /// True when the item is a single resource.
    pub is_resource: Option<bool>,
    /// Last-modified dates from header, sitemap, and meta tag.
    pub last_modified: Option<OnPageResourceHtmlLastModified>,
}
/// Meta information extracted from a crawled HTML page.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceMeta {
    /// Text of the page title tag.
    pub title: Option<String>,
    /// Code page identifier of the page charset (e.g. 65001 for UTF-8).
    pub charset: Option<i32>,
    /// True when the meta robots tag allows following links.
    pub follow: Option<bool>,
    /// Value of the generator meta tag.
    pub generator: Option<String>,
    /// Heading tags on the page keyed by level (h1, h2, …).
    pub htags: Option<HashMap<String, Option<Vec<String>>>>,
    /// Content of the meta description tag.
    pub description: Option<String>,
    /// URL of the page favicon.
    pub favicon: Option<String>,
    /// Content of the meta keywords tag.
    pub meta_keywords: Option<String>,
    /// Canonical URL declared for the page.
    pub canonical: Option<String>,
    /// Number of internal links on the page.
    pub internal_links_count: Option<i64>,
    /// Number of external links on the page.
    pub external_links_count: Option<i64>,
    /// Number of images on the page.
    pub images_count: Option<i64>,
    /// Total size of images on the page, in bytes.
    pub images_size: Option<i64>,
    /// Number of scripts on the page.
    pub scripts_count: Option<i64>,
    /// Total size of scripts on the page, in bytes.
    pub scripts_size: Option<i64>,
    /// Number of stylesheets on the page.
    pub stylesheets_count: Option<i64>,
    /// Total size of stylesheets on the page, in bytes.
    pub stylesheets_size: Option<i64>,
    /// Length of the title tag, in characters.
    pub title_length: Option<i32>,
    /// Length of the description tag, in characters.
    pub description_length: Option<i32>,
    /// Number of scripts that block page rendering.
    pub render_blocking_scripts_count: Option<i64>,
    /// Number of stylesheets that block page rendering.
    pub render_blocking_stylesheets_count: Option<i64>,
    /// Cumulative Layout Shift, a Core Web Vitals stability metric.
    pub cumulative_layout_shift: Option<f64>,
    /// Open Graph and Twitter card social media tags found on the page.
    pub social_media_tags: Option<Value>,
    /// Content readability and consistency metrics for the page.
    pub content: Option<OnPageResourceMetaContent>,
    /// Deprecated HTML tags found on the page.
    pub deprecated_tags: Option<Vec<String>>,
    /// Meta tags that appear more than once on the page.
    pub duplicate_meta_tags: Option<Vec<String>>,
    /// Spell-check results for the page content.
    pub spell: Option<OnPageSpell>,
    /// Errors and warnings encountered while parsing page resources.
    pub resource_errors: Option<OnPageResourceErrors>,
}
/// Content readability and consistency metrics for a page.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceMetaContent {
    /// Total size of the plain text on the page, in bytes.
    pub plain_text_size: Option<i64>,
    /// Ratio of plain text to total page size.
    pub plain_text_rate: Option<f64>,
    /// Number of words on the page.
    pub plain_text_word_count: Option<i64>,
    /// Automated Readability Index score.
    pub automated_readability_index: Option<f64>,
    /// Coleman–Liau Index readability score.
    pub coleman_liau_readability_index: Option<f64>,
    /// Dale–Chall readability score.
    pub dale_chall_readability_index: Option<f64>,
    /// Flesch–Kincaid readability score.
    pub flesch_kincaid_readability_index: Option<f64>,
    /// SMOG readability score.
    pub smog_readability_index: Option<f64>,
    /// Consistency of the meta description with page content (0–1).
    pub description_to_content_consistency: Option<f64>,
    /// Consistency of the title tag with page content (0–1).
    pub title_to_content_consistency: Option<f64>,
    /// Consistency of the meta keywords with page content (0–1).
    pub meta_keywords_to_content_consistency: Option<f64>,
}
/// Spell-check results for a crawled page.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageSpell {
    /// Hunspell language code used for spell checking.
    pub hunspell_language_code: Option<String>,
    /// Words detected as misspelled on the page.
    pub misspelled: Option<Vec<OnPageSpellMisspelled>>,
}
/// A single misspelled word detected on a page.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageSpellMisspelled {
    /// The misspelled word.
    pub word: Option<String>,
}
/// Errors and warnings encountered while parsing page resources.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceErrors {
    /// Errors encountered while parsing the resource.
    pub errors: Option<Vec<OnPageResourceErrorsError>>,
    /// Warnings encountered while parsing the resource.
    pub warnings: Option<Vec<OnPageResourceErrorsWarning>>,
}
/// A single error encountered while parsing a page resource.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceErrorsError {
    /// Line number where the error occurred.
    pub line: Option<i32>,
    /// Human-readable error description.
    pub message: Option<String>,
}
/// A single warning encountered while parsing a page resource.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceErrorsWarning {
    /// Line number where the warning occurred.
    pub line: Option<i32>,
    /// Human-readable warning description.
    pub message: Option<String>,
}
/// Page load timing metrics for a crawled page.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceHtmlPageTiming {
    /// Time to Interactive, in milliseconds.
    pub time_to_interactive: Option<i64>,
    /// Time until the page and its subresources finish loading, in milliseconds.
    pub dom_complete: Option<i64>,
    /// Largest Contentful Paint, a Core Web Vitals metric, in milliseconds.
    pub largest_contentful_paint: Option<f64>,
    /// First Input Delay, a Core Web Vitals responsiveness metric, in milliseconds.
    pub first_input_delay: Option<f64>,
    /// Time to connect to the server, in milliseconds.
    pub connection_time: Option<i64>,
    /// Time to establish a secure connection, in milliseconds.
    pub time_to_secure_connection: Option<i64>,
    /// Time to send the request to the server, in milliseconds.
    pub request_sent_time: Option<i64>,
    /// Time to first byte (TTFB), in milliseconds.
    pub waiting_time: Option<i64>,
    /// Time for the browser to receive the response, in milliseconds.
    pub download_time: Option<i64>,
    /// Total time until the browser receives the complete response, in milliseconds.
    pub duration_time: Option<i64>,
    /// Time to start downloading the HTML resource, in milliseconds.
    pub fetch_start: Option<i64>,
    /// Time to complete downloading the HTML resource, in milliseconds.
    pub fetch_end: Option<i64>,
}
/// Cache-control information for a crawled page.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceHtmlCacheControl {
    /// True when the page is cacheable.
    pub cachable: Option<bool>,
    /// Time to live the browser caches the resource, in milliseconds.
    pub ttl: Option<i64>,
}
/// Result of the on-page SEO checks for a crawled page.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceHtmlWebsiteChecks {
    /// True when the page has no content encoding (no compression).
    pub no_content_encoding: Option<bool>,
    /// True when the page loading time exceeds 3 seconds.
    pub high_loading_time: Option<bool>,
    /// True when the page returns a 3xx redirect status code.
    pub is_redirect: Option<bool>,
    /// True when the page returns a 4xx status code.
    pub is_4xx_code: Option<bool>,
    /// True when the page returns a 5xx status code.
    pub is_5xx_code: Option<bool>,
    /// True when the page is broken (status code below 200 or above 400).
    pub is_broken: Option<bool>,
    /// True when the page exists on the www subdomain.
    pub is_www: Option<bool>,
    /// True when the page uses the HTTPS protocol.
    pub is_https: Option<bool>,
    /// True when the page uses the HTTP protocol.
    pub is_http: Option<bool>,
    /// True when the time to first byte exceeds 1.5 seconds.
    pub high_waiting_time: Option<bool>,
    /// True when the page has no `<!DOCTYPE>` declaration.
    pub no_doctype: Option<bool>,
    /// True when the page is the canonical version.
    pub canonical: Option<bool>,
    /// True when the page has no charset encoding meta tag.
    pub no_encoding_meta_tag: Option<bool>,
    /// True when the page has empty or absent h1 tags.
    pub no_h1_tag: Option<bool>,
    /// True when an HTTPS page links to HTTP pages.
    pub https_to_http_links: Option<bool>,
    /// True when the page declares an HTML doctype.
    pub has_html_doctype: Option<bool>,
    /// True when the page size is larger than 3 MB.
    pub size_greater_than_3mb: Option<bool>,
    /// True when the declared charset and the page charset are consistent.
    pub meta_charset_consistency: Option<bool>,
    /// True when the page uses a meta http-equiv refresh redirect.
    pub has_meta_refresh_redirect: Option<bool>,
    /// True when the page has render-blocking resources.
    pub has_render_blocking_resources: Option<bool>,
    /// True when the page has a chain of at least two redirects.
    pub redirect_chain: Option<bool>,
    /// True when the plain-text-to-size ratio is below 0.1.
    pub low_content_rate: Option<bool>,
    /// True when the plain-text-to-size ratio is above 0.9.
    pub high_content_rate: Option<bool>,
    /// True when the page has fewer than 1024 characters.
    pub low_character_count: Option<bool>,
    /// True when the page has more than 256,000 characters.
    pub high_character_count: Option<bool>,
    /// True when the page is smaller than 1024 bytes.
    pub small_page_size: Option<bool>,
    /// True when the page is larger than 1 MB.
    pub large_page_size: Option<bool>,
    /// True when the Flesch–Kincaid readability score is below 15.
    pub low_readability_rate: Option<bool>,
    /// True when the meta description is irrelevant to the page content.
    pub irrelevant_description: Option<bool>,
    /// True when the title is irrelevant to the page content.
    pub irrelevant_title: Option<bool>,
    /// True when the meta keywords are irrelevant to the page content.
    pub irrelevant_meta_keywords: Option<bool>,
    /// True when the title exceeds 65 characters.
    pub title_too_long: Option<bool>,
    /// True when the title is shorter than 30 characters.
    pub title_too_short: Option<bool>,
    /// True when the page contains deprecated HTML tags.
    pub deprecated_html_tags: Option<bool>,
    /// True when the page has duplicate meta tags.
    pub duplicate_meta_tags: Option<bool>,
    /// True when the page has more than one title tag.
    pub duplicate_title_tag: Option<bool>,
    /// True when the page has images without alt attributes.
    pub no_image_alt: Option<bool>,
    /// True when the page has images without title attributes.
    pub no_image_title: Option<bool>,
    /// True when the page has an empty or absent meta description.
    pub no_description: Option<bool>,
    /// True when the page has an empty or absent title tag.
    pub no_title: Option<bool>,
    /// True when the page has no favicon.
    pub no_favicon: Option<bool>,
    /// True when the URL is SEO-friendly.
    pub seo_friendly_url: Option<bool>,
    /// True when the page contains Flash elements.
    pub flash: Option<bool>,
    /// True when the page contains frame, iframe, or frameset tags.
    pub frame: Option<bool>,
    /// True when the page contains lorem ipsum placeholder text.
    pub lorem_ipsum: Option<bool>,
    /// True when the page contains spelling errors.
    pub has_misspelling: Option<bool>,
    /// True when the URL contains only Latin characters, digits, and dashes.
    pub seo_friendly_url_characters_check: Option<bool>,
    /// True when the URL is free of dynamic parameters.
    pub seo_friendly_url_dynamic_check: Option<bool>,
    /// True when the URL is consistent with the title tag.
    pub seo_friendly_url_keywords_check: Option<bool>,
    /// True when the URL length is 120 characters or fewer.
    pub seo_friendly_url_relative_length_check: Option<bool>,
    /// True when no internal links point to the page (orphan page).
    pub is_orphan_page: Option<bool>,
    /// True when the page has both followed and nofollowed incoming internal links.
    pub is_link_relation_conflict: Option<bool>,
    /// True when the page links to pages that redirect elsewhere.
    pub has_links_to_redirects: Option<bool>,
    /// True when the canonical points to another canonical page (chain).
    pub canonical_chain: Option<bool>,
    /// True when the canonical points to a page that redirects elsewhere.
    pub canonical_to_redirect: Option<bool>,
    /// True when the canonical points to a broken page.
    pub canonical_to_broken: Option<bool>,
    /// True when the page canonicalizes back to itself (recursive canonical).
    pub recursive_canonical: Option<bool>,
}

/// Last-modified dates reported for a crawled page.
/// See <https://docs.dataforseo.com/v3/on_page/pages/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OnPageResourceHtmlLastModified {
    /// Last-modified date from the response header (UTC).
    pub header: Option<String>,
    /// Last-modified date from the sitemap (UTC).
    pub sitemap: Option<String>,
    /// Last-modified date from the page meta tag (UTC).
    pub meta_tag: Option<String>,
}
