use serde::{Deserialize, Serialize};

/// Result of `domain_analytics/technologies/technologies`.
/// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/technologies/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiAvailableTechnologies {
    /// Technology groups.
    pub groups: Option<Vec<DomainAnalyticsApiTechnologyGroup>>,
}

/// A technology group.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiTechnologyGroup {
    /// Group identifier.
    pub id: Option<String>,
    /// Group title.
    pub title: Option<String>,
    /// Categories of the group.
    pub categories: Option<Vec<DomainAnalyticsApiTechnologyCategory>>,
}

/// A technology category.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiTechnologyCategory {
    /// Category identifier.
    pub id: Option<String>,
    /// Category path.
    pub path: Option<String>,
    /// Category title.
    pub title: Option<String>,
    /// Technology names.
    pub technologies: Option<Vec<String>>,
}
