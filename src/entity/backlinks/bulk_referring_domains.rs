use crate::entity::BacklinksApiElementBulkReferringDomains;
use serde::{Deserialize, Serialize};

/// Result of the Bulk Referring Domains endpoint: referring-domain counts for up to 1000 targets.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_referring_domains/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkReferringDomains {
    /// Number of elements in the `items` array.
    pub items_count: Option<i32>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBulkReferringDomains>>,
}
