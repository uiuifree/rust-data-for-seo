use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of the Summary endpoint: aggregated backlink metrics for a target.
/// See <https://docs.dataforseo.com/v3/backlinks/summary/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiSummary {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Date and time the element was first found (UTC).
    pub first_seen: Option<String>,
    /// Date and time the element was last seen before being lost (UTC).
    pub lost_date: Option<String>,
    /// Rank of this element on the configured scale (0-1000 by default).
    pub rank: Option<i32>,
    /// Number of backlinks pointing to the target.
    pub backlinks: Option<i32>,
    /// Average spam score of the backlinks (0-100).
    pub backlinks_spam_score: Option<i32>,
    /// Number of target pages crawled.
    pub crawled_pages: Option<i32>,
    /// Metadata about the target.
    pub info: Option<BacklinksApiSummaryInfo>,
    /// Number of internal links on the target.
    pub internal_links_count: Option<i32>,
    /// Number of external links on the target.
    pub external_links_count: Option<i32>,
    /// Number of broken backlinks.
    pub broken_backlinks: Option<i32>,
    /// Number of referring pages returning a 4xx/5xx status.
    pub broken_pages: Option<i32>,
    /// Number of referring domains pointing to the target.
    pub referring_domains: Option<i32>,
    /// Number of referring domains with at least one nofollow link.
    pub referring_domains_nofollow: Option<i32>,
    /// Number of referring main (root) domains.
    pub referring_main_domains: Option<i32>,
    /// Number of referring main domains with at least one nofollow link.
    pub referring_main_domains_nofollow: Option<i32>,
    /// Number of referring IP addresses.
    pub referring_ips: Option<i32>,
    /// Number of referring subnetworks.
    pub referring_subnets: Option<i32>,
    /// Number of referring pages pointing to the target.
    pub referring_pages: Option<i32>,
    /// Distribution of referring links by top-level domain with counts.
    pub referring_links_tld: Option<HashMap<String, i32>>,
    /// Distribution of referring link types (anchor, image, link, ...) with counts.
    pub referring_links_types: Option<HashMap<String, i32>>,
    /// Distribution of referring link attributes (nofollow, ugc, ...) with counts.
    pub referring_links_attributes: Option<HashMap<String, i32>>,
    /// Distribution of referring links by platform type with counts.
    pub referring_links_platform_types: Option<HashMap<String, i32>>,
    /// Distribution of referring links by HTML semantic location with counts.
    pub referring_links_semantic_locations: Option<HashMap<String, i32>>,
    /// Distribution of referring links by ISO country code with counts.
    pub referring_links_countries: Option<HashMap<String, i32>>,
    /// Number of referring pages with at least one nofollow link.
    pub referring_pages_nofollow: Option<i32>,
}

/// Metadata about the target (server, CMS, IP, country, spam score).
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiSummaryInfo {
    /// Server software reported by the target.
    pub server: Option<String>,
    /// Content management system detected for the target.
    pub cms: Option<String>,
    /// Platform types detected for the target.
    pub platform_type: Option<Vec<String>>,
    /// IP address of the target.
    pub ip_address: Option<String>,
    /// Country code of the target.
    pub country: Option<String>,
    /// Whether the target is an IP address.
    pub is_ip: Option<bool>,
    /// Average spam score of the target (0-100).
    pub target_spam_score: Option<i32>,
}
