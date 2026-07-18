use crate::entity::ContentAnalysisApiSummary;
use serde::{Deserialize, Serialize};

/// Result of `content_analysis/rating_distribution/live`: citation metrics for
/// a bucket of the content-rating scale.
/// See <https://docs.dataforseo.com/v3/content_analysis/rating_distribution/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiRatingDistribution {
    /// Element type.
    #[serde(rename = "type")]
    pub element_type: Option<String>,
    /// Minimum rating of this bucket on the distribution scale.
    pub min: Option<f64>,
    /// Maximum rating of this bucket on the distribution scale.
    pub max: Option<f64>,
    /// Citation metrics for this rating bucket.
    pub metrics: Option<ContentAnalysisApiSummary>,
}
