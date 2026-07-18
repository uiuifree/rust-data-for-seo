use serde::{Deserialize, Serialize};

/// A single backlink-count item returned by the Bulk Backlinks endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_backlinks/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBulkBacklinks {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Number of backlinks pointing to the target.
    pub backlinks: Option<i64>,
}
