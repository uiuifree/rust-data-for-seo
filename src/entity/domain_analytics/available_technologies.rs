use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result object for the `available_technologies` endpoint.
///
/// The full catalogue of detectable technologies, structured as a dynamic
/// hierarchy of technology group → category → list of technology names.
/// See <https://docs.dataforseo.com/v3/domain_analytics/technologies/technologies/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DomainAnalyticsApiAvailableTechnologies {
    /// Technology group name → category name → list of technology names.
    #[serde(flatten)]
    pub groups: HashMap<String, HashMap<String, Vec<String>>>,
}
