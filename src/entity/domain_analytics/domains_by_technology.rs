use crate::entity::DomainAnalyticsApiDomainTechnologyItem;
use serde::{Deserialize, Serialize};

/// Result object for the `domains_by_technology` endpoint.
/// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/domains_by_technology/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiDomainsByTechnology {
    /// Total number of domains matching the request across all pages.
    pub total_count: Option<i64>,
    /// Number of domains returned in this response.
    pub items_count: Option<i64>,
    /// Offset applied to the returned results.
    pub offset: Option<i32>,
    /// Pagination token to fetch the next batch of results.
    pub offset_token: Option<String>,
    /// The matching domain profiles.
    pub items: Option<Vec<DomainAnalyticsApiDomainTechnologyItem>>,
}
