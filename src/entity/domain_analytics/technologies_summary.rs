use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result object for the `technologies_summary` endpoint.
///
/// Each map associates a code or phrase with the number of matching websites.
/// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/technologies_summary/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiTechnologiesSummary {
    /// Total number of domains matching the request.
    pub total_count: Option<i64>,
    /// Number of aggregated entries returned.
    pub items_count: Option<i64>,
    /// Matching-website count per ISO country code.
    pub countries: Option<HashMap<String, i64>>,
    /// Matching-website count per language code.
    pub languages: Option<HashMap<String, i64>>,
    /// Matching-website count per content-language code.
    pub content_languages: Option<HashMap<String, i64>>,
    /// Matching-website count per domain keyword phrase.
    pub keywords: Option<HashMap<String, i64>>,
}
