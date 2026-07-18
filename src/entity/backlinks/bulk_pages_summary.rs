use crate::entity::BacklinksApiElementBacklinksPageSummary;
use serde::{Deserialize, Serialize};

/// Result of the Bulk Pages Summary endpoint: backlink summary for up to 1000 pages.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_pages_summary/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkPagesSummary {
    /// Total number of relevant elements in the database.
    pub total_count: Option<i64>,
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBacklinksPageSummary>>,
}
