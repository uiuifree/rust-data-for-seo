use crate::entity::OnPageDataApiCrawlStatus;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result object for the OnPage Summary endpoint.
/// See <https://docs.dataforseo.com/v3/on_page/summary/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageDataApiSummary {
    /// Crawling progress: "in_progress" or "finished".
    pub crawl_progress: Option<String>,
    /// Statistics on the crawling session.
    pub crawl_status: Option<OnPageDataApiCrawlStatus>,
    /// IP address of the gateway used for crawling.
    pub crawl_gateway_address: Option<String>,
    /// Reason the crawl stopped, if it ended early.
    pub crawl_stop_reason: Option<String>,
    /// Information about the crawled domain.
    pub domain_info: Option<OnPageSummaryDomainInfo>,
    /// Aggregated on-page metrics across all crawled pages.
    pub page_metrics: Option<OnPageSummaryPageMetrics>,
}

/// General information about the crawled domain.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageSummaryDomainInfo {
    /// Domain name that was crawled.
    pub name: Option<String>,
    /// Content management system detected on the domain.
    pub cms: Option<String>,
    /// IP address of the domain's server.
    pub ip: Option<String>,
    /// Value of the `Server` response header.
    pub server: Option<String>,
    /// Timestamp when the crawl started (UTC).
    pub crawl_start: Option<String>,
    /// Timestamp when the crawl finished (UTC).
    pub crawl_end: Option<String>,
    /// Detailed status of the crawl, e.g. "finished".
    pub extended_crawl_status: Option<String>,
    /// SSL certificate information for the domain.
    pub ssl_info: Option<OnPageSummarySslInfo>,
    /// Boolean flags for domain-level on-page checks.
    pub checks: Option<Value>,
    /// Total number of pages found on the domain.
    pub total_pages: Option<i64>,
    /// HTTP status code returned for a non-existent page.
    pub page_not_found_status_code: Option<i32>,
    /// HTTP status code returned for the canonicalization check.
    pub canonicalization_status_code: Option<i32>,
    /// Main (root) domain name.
    pub main_domain: Option<String>,
}

/// SSL certificate details for the crawled domain.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageSummarySslInfo {
    /// Whether the SSL certificate is valid.
    pub valid_certificate: Option<bool>,
    /// Organization that issued the certificate.
    pub certificate_issuer: Option<String>,
    /// Subject the certificate was issued to.
    pub certificate_subject: Option<String>,
    /// Version of the certificate.
    pub certificate_version: Option<String>,
    /// Hash of the certificate.
    pub certificate_hash: Option<String>,
    /// Expiration date of the certificate (UTC).
    pub certificate_expiration_date: Option<String>,
}

/// Aggregated on-page metrics across all crawled pages.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct OnPageSummaryPageMetrics {
    /// Number of external links across all pages.
    pub links_external: Option<i64>,
    /// Number of internal links across all pages.
    pub links_internal: Option<i64>,
    /// Number of pages with duplicate title tags.
    pub duplicate_title: Option<i64>,
    /// Number of pages with duplicate meta descriptions.
    pub duplicate_description: Option<i64>,
    /// Number of pages with duplicate content.
    pub duplicate_content: Option<i64>,
    /// Number of broken links found.
    pub broken_links: Option<i64>,
    /// Number of broken resources found.
    pub broken_resources: Option<i64>,
    /// Number of links with conflicting rel attributes.
    pub links_relation_conflict: Option<i64>,
    /// Number of redirect loops detected.
    pub redirect_loop: Option<i64>,
    /// Overall on-page score for the domain, 0–100.
    pub onpage_score: Option<f64>,
    /// Number of non-indexable pages.
    pub non_indexable: Option<i64>,
    /// Aggregated counts of on-page checks across all pages.
    pub checks: Option<Value>,
}
