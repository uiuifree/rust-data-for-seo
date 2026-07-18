use crate::entity::BacklinksApiElementBulkRanks;
use serde::{Deserialize, Serialize};

/// Result of the Bulk Ranks endpoint: rank for up to 1000 targets.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_ranks/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkRanks {
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBulkRanks>>,
}
