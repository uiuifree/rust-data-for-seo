use serde::{Deserialize, Serialize};

/// A single month of clickstream-based historical search volume.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiClickstreamMonthlySearch {
    /// Calendar year.
    pub year: Option<i32>,
    /// Calendar month (1-12).
    pub month: Option<i32>,
    /// Search volume for the month.
    pub search_volume: Option<i64>,
}

/// A result item from the Clickstream `dataforseo_search_volume` endpoint.
/// See <https://docs.dataforseo.com/v3/keywords_data/clickstream_data/dataforseo_search_volume/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiClickstreamSearchVolume {
    /// The keyword this result refers to.
    pub keyword: Option<String>,
    /// Location code echoed from the request.
    pub location_code: Option<i32>,
    /// Language code echoed from the request.
    pub language_code: Option<String>,
    /// Average monthly search volume, blended with clickstream data.
    pub search_volume: Option<i64>,
    /// Whether clickstream data was blended into the search volume.
    pub use_clickstream: Option<bool>,
    /// Search volume broken down by month.
    pub monthly_searches: Option<Vec<KeywordsDataApiClickstreamMonthlySearch>>,
}

/// A result item from the Clickstream `global_search_volume` endpoint.
/// See <https://docs.dataforseo.com/v3/keywords_data/clickstream_data/global_search_volume/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiClickstreamGlobalSearchVolume {
    /// The keyword this result refers to (lowercased).
    pub keyword: Option<String>,
    /// Worldwide average monthly search volume.
    pub search_volume: Option<i64>,
    /// Per-country share of the worldwide search volume.
    pub country_distribution: Option<Vec<KeywordsDataApiClickstreamCountryDistribution>>,
}

/// A per-country share of a keyword's global search volume.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiClickstreamCountryDistribution {
    /// ISO country code, e.g. "US".
    pub country_iso_code: Option<String>,
    /// Search volume attributed to the country.
    pub search_volume: Option<i64>,
    /// Percentage of the worldwide search volume from this country.
    pub percentage: Option<f64>,
}

/// A result item from the Clickstream `bulk_search_volume` endpoint.
/// See <https://docs.dataforseo.com/v3/keywords_data/clickstream_data/bulk_search_volume/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiClickstreamBulkSearchVolume {
    /// The keyword this result refers to.
    pub keyword: Option<String>,
    /// Average monthly search volume.
    pub search_volume: Option<i64>,
    /// Search volume broken down by month.
    pub monthly_searches: Option<Vec<KeywordsDataApiClickstreamMonthlySearch>>,
}

/// A location with its available languages, returned by the Clickstream
/// `locations_and_languages` endpoint.
/// See <https://docs.dataforseo.com/v3/keywords_data/clickstream_data/locations_and_languages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiClickstreamLocation {
    /// Numeric location identifier.
    pub location_code: Option<i32>,
    /// Full location name.
    pub location_name: Option<String>,
    /// Code of the parent location, if any.
    pub location_code_parent: Option<i32>,
    /// ISO country code of the location.
    pub country_iso_code: Option<String>,
    /// Location type, e.g. "Country" or "Region".
    pub location_type: Option<String>,
    /// Languages available for this location.
    pub available_languages: Option<Vec<crate::entity::KeywordsDataApiLanguage>>,
}
