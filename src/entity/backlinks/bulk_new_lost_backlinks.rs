use crate::entity::BacklinksApiElementBulkNewLostBacklinks;
use serde::{Deserialize, Serialize};

/// Result of the Bulk New & Lost Backlinks endpoint for up to 1000 targets.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_new_lost_backlinks/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkNewLostBacklinks {
    /// Number of elements in the `items` array.
    pub items_count: Option<i32>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBulkNewLostBacklinks>>,
}
