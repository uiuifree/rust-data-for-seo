use crate::entity::{ContentAnalysisCategoryCount, ContentAnalysisTopDomain};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Aggregated citation metrics for a keyword.
///
/// Returned by `content_analysis/summary/live` and reused as the metric block
/// inside sentiment analysis, rating distribution, and trend results.
/// See <https://docs.dataforseo.com/v3/content_analysis/summary/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiSummary {
    /// Element type, e.g. `content_analysis_summary`.
    #[serde(rename = "type")]
    pub element_type: Option<String>,
    /// Total number of matching citations in the database.
    pub total_count: Option<i64>,
    /// Normalized sum of the ranks of all URLs citing the keyword.
    pub rank: Option<i64>,
    /// Top citing domains, each with its citation count.
    pub top_domains: Option<Vec<ContentAnalysisTopDomain>>,
    /// Citation count per sentiment connotation (anger, happiness, love,
    /// sadness, share, fun).
    pub sentiment_connotations: Option<HashMap<String, f64>>,
    /// Citation count per sentiment polarity (positive, negative, neutral).
    pub connotation_types: Option<HashMap<String, f64>>,
    /// Text categories, each with its citation count.
    pub text_categories: Option<Vec<ContentAnalysisCategoryCount>>,
    /// Page categories, each with its citation count.
    pub page_categories: Option<Vec<ContentAnalysisCategoryCount>>,
    /// Citation count per page type (ecommerce, news, blogs, message-boards,
    /// organization).
    pub page_types: Option<HashMap<String, i64>>,
    /// Citation count per country, keyed by ISO country code.
    pub countries: Option<HashMap<String, i64>>,
    /// Citation count per language, keyed by language code.
    pub languages: Option<HashMap<String, i64>>,
}
