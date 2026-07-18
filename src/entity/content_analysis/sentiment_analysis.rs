use crate::entity::ContentAnalysisApiSummary;
use serde::{Deserialize, Serialize};

/// Result of `content_analysis/sentiment_analysis/live`: citation stats split
/// by sentiment polarity and by sentiment connotation.
/// See <https://docs.dataforseo.com/v3/content_analysis/sentiment_analysis/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiSentimentAnalysis {
    /// Element type.
    #[serde(rename = "type")]
    pub element_type: Option<String>,
    /// Citation metrics grouped by sentiment polarity.
    pub positive_connotation_distribution: Option<ContentAnalysisPositiveConnotationDistribution>,
    /// Citation metrics grouped by sentiment connotation.
    pub sentiment_connotation_distribution: Option<ContentAnalysisSentimentConnotationDistribution>,
}

/// Citation metrics grouped by sentiment polarity.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisPositiveConnotationDistribution {
    /// Metrics for citations with positive sentiment.
    pub positive: Option<ContentAnalysisApiSummary>,
    /// Metrics for citations with negative sentiment.
    pub negative: Option<ContentAnalysisApiSummary>,
    /// Metrics for citations with neutral sentiment.
    pub neutral: Option<ContentAnalysisApiSummary>,
}

/// Citation metrics grouped by sentiment connotation.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisSentimentConnotationDistribution {
    /// Metrics for citations connoting anger.
    pub anger: Option<ContentAnalysisApiSummary>,
    /// Metrics for citations connoting happiness.
    pub happiness: Option<ContentAnalysisApiSummary>,
    /// Metrics for citations connoting love.
    pub love: Option<ContentAnalysisApiSummary>,
    /// Metrics for citations connoting sadness.
    pub sadness: Option<ContentAnalysisApiSummary>,
    /// Metrics for citations connoting a share impulse.
    pub share: Option<ContentAnalysisApiSummary>,
    /// Metrics for citations connoting fun.
    pub fun: Option<ContentAnalysisApiSummary>,
}
