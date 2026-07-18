use serde::{Deserialize, Serialize};

/// A keyword result item from the Google Ads Search Volume endpoint.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/search_volume/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsSearchVolumeTask {
    /// The keyword this result refers to.
    pub keyword: Option<String>,
    /// Spelling correction Google applied to the keyword, if any.
    pub spell: Option<String>,
    /// Location code echoed from the request.
    pub location_code: Option<u32>,
    /// Language code echoed from the request.
    pub language_code: Option<String>,
    /// Whether search/partner network data was included.
    pub search_partners: Option<bool>,
    /// Competition level: "LOW", "MEDIUM" or "HIGH".
    pub competition: Option<String>,
    /// Competition as an index from 0 to 100.
    pub competition_index: Option<i32>,
    /// Average monthly search volume over the last 12 months.
    pub search_volume: Option<i32>,
    /// Lower range of the top-of-page bid, in USD.
    pub low_top_of_page_bid: Option<f32>,
    /// Upper range of the top-of-page bid, in USD.
    pub high_top_of_page_bid: Option<f32>,
    /// Average cost per click, in USD.
    pub cpc: Option<f32>,
    /// Search volume broken down by month.
    pub monthly_searches: Option<Vec<KeywordsDataApiGoogleAdsSearchVolumeTaskMonthlySearch>>,
}

/// A single month of historical search volume.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsSearchVolumeTaskMonthlySearch {
    /// Calendar year.
    pub year: Option<i32>,
    /// Calendar month (1-12).
    pub month: Option<i32>,
    /// Search volume for the month.
    pub search_volume: Option<i32>,
}
