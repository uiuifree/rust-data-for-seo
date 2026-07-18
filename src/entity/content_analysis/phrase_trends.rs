use crate::entity::ContentAnalysisApiSummary;
use serde::{Deserialize, Serialize};

/// Result of `content_analysis/phrase_trends/live`: citation metrics for a
/// keyword on a single date within the requested range.
/// See <https://docs.dataforseo.com/v3/content_analysis/phrase_trends/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiPhraseTrends {
    /// Date the metrics cover.
    pub date: Option<String>,
    /// Citation metrics for the keyword on this date.
    #[serde(flatten)]
    pub metrics: ContentAnalysisApiSummary,
}
