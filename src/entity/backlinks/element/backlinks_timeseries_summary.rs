use serde::{Deserialize, Serialize};

/// A single data point returned by the Timeseries Summary endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/timeseries_summary/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBacklinksTimeseriesSummary {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Date and time of this data point (UTC).
    pub date: Option<String>,
    /// Rank of this element on the configured scale (0-1000 by default).
    pub rank: Option<i32>,
    /// Number of backlinks pointing to the target.
    pub backlinks: Option<i32>,
    /// Number of nofollow backlinks.
    pub backlinks_nofollow: Option<i32>,
    /// Number of referring pages pointing to the target.
    pub referring_pages: Option<i32>,
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
    /// Number of referring pages with at least one nofollow link.
    pub referring_pages_nofollow: Option<i32>,
}
