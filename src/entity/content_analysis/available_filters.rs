use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of `content_analysis/available_filters`: the fields that can be used
/// in `filters` / `initial_dataset_filters`, mapped to their value type.
/// See <https://docs.dataforseo.com/v3/content_analysis/available_filters/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiAvailableFilters {
    /// Filterable fields of the Search endpoint, mapped to their value type.
    pub search: Option<HashMap<String, String>>,
}
