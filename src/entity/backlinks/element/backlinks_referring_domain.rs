use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single referring-domain item returned by the Referring Domains endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/referring_domains/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksReferringDomain {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Referring domain name.
    pub domain: Option<String>,
    /// Rank of this element on the configured scale (0-1000 by default).
    pub rank: Option<i64>,
    /// Number of backlinks pointing to the target.
    pub backlinks: Option<i64>,
    /// Date and time the element was first found (UTC).
    pub first_seen: Option<String>,
    /// Date and time the element was last seen before being lost (UTC).
    pub lost_date: Option<String>,
    /// Average spam score of the backlinks (0-100).
    pub backlinks_spam_score: Option<i64>,
    /// Number of broken backlinks.
    pub broken_backlinks: Option<i64>,
    /// Number of referring pages returning a 4xx/5xx status.
    pub broken_pages: Option<i64>,
    /// Number of referring domains pointing to the target.
    pub referring_domains: Option<i64>,
    /// Number of referring domains with at least one nofollow link.
    pub referring_domains_nofollow: Option<i64>,
    /// Number of referring main (root) domains.
    pub referring_main_domains: Option<i64>,
    /// Number of referring main domains with at least one nofollow link.
    pub referring_main_domains_nofollow: Option<i64>,
    /// Number of referring IP addresses.
    pub referring_ips: Option<i64>,
    /// Number of referring subnetworks.
    pub referring_subnets: Option<i64>,
    /// Number of referring pages pointing to the target.
    pub referring_pages: Option<i64>,
    /// Distribution of referring links by top-level domain with counts.
    pub referring_links_tld: Option<HashMap<String, i64>>,
    /// Distribution of referring link types (anchor, image, link, ...) with counts.
    pub referring_links_types: Option<HashMap<String, i64>>,
    /// Distribution of referring link attributes (nofollow, ugc, ...) with counts.
    pub referring_links_attributes: Option<HashMap<String, i64>>,
    /// Distribution of referring links by platform type with counts.
    pub referring_links_platform_types: Option<HashMap<String, i64>>,
    /// Distribution of referring links by HTML semantic location with counts.
    pub referring_links_semantic_locations: Option<HashMap<String, i64>>,
    /// Distribution of referring links by ISO country code with counts.
    pub referring_links_countries: Option<HashMap<String, i64>>,
    /// Number of referring pages with at least one nofollow link.
    pub referring_pages_nofollow: Option<i64>,
}
