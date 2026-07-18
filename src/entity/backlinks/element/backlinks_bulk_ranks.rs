use serde::{Deserialize, Serialize};

/// A single rank item returned by the Bulk Ranks endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_ranks/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBulkRanks {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Rank of this element on the configured scale (0-1000 by default).
    pub rank: Option<i32>,
}
