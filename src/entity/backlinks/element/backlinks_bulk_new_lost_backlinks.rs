use serde::{Deserialize, Serialize};

/// A single new/lost backlink-count item returned by the Bulk New & Lost Backlinks endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_new_lost_backlinks/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBulkNewLostBacklinks {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Number of newly discovered backlinks.
    pub new_backlinks: Option<i64>,
    /// Number of backlinks that were lost.
    pub lost_backlinks: Option<i64>,
}
