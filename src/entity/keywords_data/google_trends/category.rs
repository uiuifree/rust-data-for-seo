use serde::{Deserialize, Serialize};

/// A Google Trends category.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_trends/categories/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleTrendsCategory {
    /// Unique Google Trends category identifier.
    pub category_code: Option<i32>,
    /// Name of the category.
    pub category_name: Option<String>,
    /// Code of the parent category; `0` for top-level categories.
    pub category_code_parent: Option<i32>,
}
