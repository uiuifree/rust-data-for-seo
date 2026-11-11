mod audience_estimation;
mod keyword;
mod keyword_performance;
mod keyword_suggestions_for_url;
mod search_volume_history;

pub use audience_estimation::*;
pub use keyword::*;
pub use keyword_performance::*;
pub use keyword_suggestions_for_url::*;
pub use search_volume_history::*;

/// A language and its locations for Bing Keyword Performance.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/keyword_performance/locations_and_languages/>.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct KeywordsDataApiBingLocationsAndLanguages {
    /// Language name.
    pub language_name: Option<String>,
    /// Language code.
    pub language_code: Option<String>,
    /// Locations supported with this language.
    pub available_locations: Option<Vec<KeywordsDataApiBingAvailableLocation>>,
}

/// A location supported by Bing Keyword Performance.
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
pub struct KeywordsDataApiBingAvailableLocation {
    /// Location code.
    pub location_code: Option<i32>,
    /// Location name.
    pub location_name: Option<String>,
    /// ISO country code.
    pub country_iso_code: Option<String>,
    /// Location type.
    pub location_type: Option<String>,
}
