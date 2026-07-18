use crate::entity::BacklinksApiSummaryInfo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of the History endpoint: historical backlink metrics for a target.
/// See <https://docs.dataforseo.com/v3/backlinks/history/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiHistory {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Start date of the range (`yyyy-mm-dd`).
    pub date_from: Option<String>,
    /// End date of the range (`yyyy-mm-dd`).
    pub date_to: Option<String>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i32>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiHistoryItem>>,
}

/// A single monthly data point in a History result.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiHistoryItem {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Date and time of this data point (UTC).
    pub date: Option<String>,
    /// Rank of this element on the configured scale (0-1000 by default).
    pub rank: Option<i32>,
    /// Number of backlinks pointing to the target.
    pub backlinks: Option<i32>,
    /// Number of newly discovered backlinks.
    pub new_backlinks: Option<i32>,
    /// Number of backlinks that were lost.
    pub lost_backlinks: Option<i32>,
    /// Number of newly discovered referring domains.
    pub new_referring_domains: Option<i32>,
    /// Number of referring domains that were lost.
    pub lost_referring_domains: Option<i32>,
    /// Number of target pages crawled.
    pub crawled_pages: Option<i32>,
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
    /// Number of referring pages with at least one nofollow link.
    pub referring_pages_nofollow: Option<i32>,
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
    /// Metadata about the target.
    pub info: Option<BacklinksApiSummaryInfo>,
}
