use crate::entity::BacklinksApiElementBulkNewLostReferringDomains;
use serde::{Deserialize, Serialize};

/// Result of the Bulk New & Lost Referring Domains endpoint for up to 1000 targets.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_new_lost_referring_domains/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkNewLostReferringDomains {
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBulkNewLostReferringDomains>>,
}
