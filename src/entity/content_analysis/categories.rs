use serde::{Deserialize, Serialize};

/// Result item of `content_analysis/categories`: a category in the
/// content-analysis taxonomy.
/// See <https://docs.dataforseo.com/v3/content_analysis/categories/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiCategory {
    /// Category code.
    pub category_code: Option<i64>,
    /// Full category name.
    pub category_name: Option<String>,
    /// Code of the parent category, if any.
    pub category_code_parent: Option<i64>,
}
