use serde::{Deserialize, Serialize};

/// A result item from the Bing `search_volume_history` endpoint.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/search_volume_history/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingSearchVolumeHistory {
    /// The keyword this history refers to.
    pub keyword: Option<String>,
    /// Location code echoed from the request.
    pub location_code: Option<i32>,
    /// Language code echoed from the request.
    pub language_code: Option<String>,
    /// Device filters the history was requested for.
    pub device: Option<Vec<String>>,
    /// Aggregation period: "monthly", "weekly" or "daily".
    pub period: Option<String>,
    /// Historical search volume series, split by device.
    pub searches: Option<KeywordsDataApiBingSearchVolumeHistorySearches>,
}

/// Per-device historical search volume series.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingSearchVolumeHistorySearches {
    /// History for desktop devices.
    pub desktop: Option<Vec<KeywordsDataApiBingSearchVolumeHistoryRecord>>,
    /// History for mobile devices.
    pub mobile: Option<Vec<KeywordsDataApiBingSearchVolumeHistoryRecord>>,
    /// History for tablet devices.
    pub tablet: Option<Vec<KeywordsDataApiBingSearchVolumeHistoryRecord>>,
    /// History for non-smartphone devices.
    pub non_smartphones: Option<Vec<KeywordsDataApiBingSearchVolumeHistoryRecord>>,
}

/// A single historical search volume record.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingSearchVolumeHistoryRecord {
    /// Calendar year.
    pub year: Option<i32>,
    /// Calendar month (1-12).
    pub month: Option<i32>,
    /// Day of the month.
    pub day: Option<i32>,
    /// Search volume for the record.
    pub search_volume: Option<i64>,
}

/// A language with its available locations, returned by the Bing
/// `search_volume_history/locations_and_languages` endpoint. Per the docs the
/// response is keyed by language, each carrying its supported locations.
/// See <https://docs.dataforseo.com/v3/keywords_data/bing/search_volume_history/locations_and_languages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingSearchVolumeHistoryLocationLanguage {
    /// Full language name.
    pub language_name: Option<String>,
    /// ISO 639-1 language code.
    pub language_code: Option<String>,
    /// Locations available for this language.
    pub available_locations: Option<Vec<KeywordsDataApiBingSearchVolumeHistoryLocation>>,
}

/// A location entry nested under a `search_volume_history` language.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiBingSearchVolumeHistoryLocation {
    /// Numeric location identifier.
    pub location_code: Option<i32>,
    /// Full location name.
    pub location_name: Option<String>,
    /// ISO country code of the location.
    pub country_iso_code: Option<String>,
    /// Location type, e.g. "Country" or "Region".
    pub location_type: Option<String>,
}
