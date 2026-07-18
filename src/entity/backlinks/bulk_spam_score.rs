use crate::entity::BacklinksApiElementBulkSpamScore;
use serde::{Deserialize, Serialize};

/// Result of the Bulk Spam Score endpoint: spam score for up to 1000 targets.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_spam_score/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiBulkSpamScore {
    /// Number of elements in the `items` array.
    pub items_count: Option<i64>,
    /// Elements returned for this result.
    pub items: Option<Vec<BacklinksApiElementBulkSpamScore>>,
}
