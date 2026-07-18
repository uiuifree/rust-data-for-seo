use serde::{Deserialize, Serialize};

/// A single spam-score item returned by the Bulk Spam Score endpoint.
/// See <https://docs.dataforseo.com/v3/backlinks/bulk_spam_score/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BacklinksApiElementBulkSpamScore {
    /// Element type identifier for this item.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Target the metrics refer to (domain, subdomain, or webpage).
    pub target: Option<String>,
    /// Spam score of the target (0-100).
    pub spam_score: Option<i32>,
}
