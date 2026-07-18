use serde::{Deserialize, Serialize};

/// A keyword result item shared by the Bing `search_volume`, `keywords_for_site`
/// and `keywords_for_keywords` endpoints.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/search_volume/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingKeyword {
    /// The keyword this result refers to.
    pub keyword: Option<String>,
    /// Location code echoed from the request.
    pub location_code: Option<i32>,
    /// Language code echoed from the request.
    pub language_code: Option<String>,
    /// Whether search/partner network data was included.
    pub search_partners: Option<bool>,
    /// Device filter applied, e.g. "all", "mobile", "desktop" or "tablet".
    pub device: Option<String>,
    /// Relative competition from 0.0 (low) to 1.0 (high).
    pub competition: Option<f32>,
    /// Average cost per click, in USD.
    pub cpc: Option<f32>,
    /// Average monthly search volume.
    pub search_volume: Option<i32>,
    /// Product/service category identifiers (legacy, usually null).
    pub categories: Option<Vec<i64>>,
    /// Search volume broken down by month.
    pub monthly_searches: Option<Vec<KeywordsDataApiBingMonthlySearch>>,
}

/// A single month of historical search volume.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingMonthlySearch {
    /// Calendar year.
    pub year: Option<i32>,
    /// Calendar month (1-12).
    pub month: Option<i32>,
    /// Search volume for the month.
    pub search_volume: Option<i32>,
}
