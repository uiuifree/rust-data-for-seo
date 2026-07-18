use crate::entity::BacklinksElementBacklinkAnchor;
use serde::{Deserialize, Serialize};

/// Result of the Anchors endpoint: anchor texts referring to a target.
/// See <https://docs.dataforseo.com/v3/backlinks/anchors/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiAnchor {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Total number of relevant elements in the database.
    pub total_count: Option<i64>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksElementBacklinkAnchor>>,
}
