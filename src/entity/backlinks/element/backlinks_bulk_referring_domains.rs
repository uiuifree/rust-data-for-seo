use serde::{Deserialize, Serialize};

/// A single referring-domain-count item returned by the Bulk Referring Domains endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_referring_domains/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBulkReferringDomains {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Number of referring domains pointing to the target.
    pub referring_domains: Option<i32>,
    /// Number of referring domains with at least one nofollow link.
    pub referring_domains_nofollow: Option<i32>,
    /// Number of referring main (root) domains.
    pub referring_main_domains: Option<i32>,
    /// Number of referring main domains with at least one nofollow link.
    pub referring_main_domains_nofollow: Option<i32>,
}
