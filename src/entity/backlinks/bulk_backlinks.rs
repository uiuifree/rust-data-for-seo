use crate::entity::BacklinksApiElementBulkBacklinks;
use serde::{Deserialize, Serialize};

/// Result of the Bulk Backlinks endpoint: backlink counts for up to 1000 targets.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_backlinks/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkBacklinks {
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBulkBacklinks>>,
}
