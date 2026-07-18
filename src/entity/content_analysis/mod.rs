//! Entity models for the content_analysis API domain.
//!
//! Mirrors <https://docs.dataforseo.com/v3/content_analysis/overview/>.

mod available_filters;
mod categories;
mod category_trends;
mod id_list;
mod languages;
mod locations;
mod phrase_trends;
mod rating_distribution;
mod search;
mod sentiment_analysis;
mod summary;

pub use available_filters::*;
pub use categories::*;
pub use category_trends::*;
pub use id_list::*;
pub use languages::*;
pub use locations::*;
pub use phrase_trends::*;
pub use rating_distribution::*;
pub use search::*;
pub use sentiment_analysis::*;
pub use summary::*;

use serde::{Deserialize, Serialize};

/// A domain and how many citations it holds, as returned in `top_domains`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisTopDomain {
    /// Domain name.
    pub domain: Option<String>,
    /// Number of citations found on this domain.
    pub count: Option<i64>,
}

/// A category (chain of category IDs) with its citation count.
/// Used by the `text_categories` and `page_categories` fields.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisCategoryCount {
    /// Category ID chain, from the broadest to the narrowest category.
    pub category: Option<Vec<i64>>,
    /// Number of citations in this category.
    pub count: Option<i64>,
}
