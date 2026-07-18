use crate::entity::BacklinksElementBacklink;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of the Backlinks endpoint: the list of backlinks pointing to a target.
/// See <https://docs.dataforseo.com/v3/backlinks/backlinks/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBacklinks {
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Grouping mode: `as_is`, `one_per_domain`, or `one_per_anchor`.
    pub mode: Option<String>,
    /// Custom grouping definition with `field` and `value`.
    pub custom_mode: Option<Value>,
    /// Total number of relevant elements in the database.
    pub total_count: Option<i64>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Pagination token for retrieving results beyond the offset limit.
    pub search_after_token: Option<String>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksElementBacklink>>,
}
