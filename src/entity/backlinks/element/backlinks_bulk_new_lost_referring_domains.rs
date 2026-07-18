use serde::{Deserialize, Serialize};

/// A single new/lost referring-domain item returned by the Bulk New & Lost Referring Domains endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_new_lost_referring_domains/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBulkNewLostReferringDomains {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Number of newly discovered referring domains.
    pub new_referring_domains: Option<i64>,
    /// Number of referring domains that were lost.
    pub lost_referring_domains: Option<i64>,
    /// Number of newly discovered referring main domains.
    pub new_referring_main_domains: Option<i64>,
    /// Number of referring main domains that were lost.
    pub lost_referring_main_domains: Option<i64>,
}
