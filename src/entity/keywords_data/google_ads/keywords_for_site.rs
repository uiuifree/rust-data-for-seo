use serde::{Deserialize, Serialize};

/// A keyword result item from the Google Ads Keywords For Site endpoint.
/// See <https://docs.dataforseo.com/v3/keywords_data/google_ads/keywords_for_site/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTask {
    /// The keyword this result refers to.
    pub keyword: Option<String>,
    /// Location code echoed from the request.
    pub location_code: Option<i32>,
    /// Language code echoed from the request.
    pub language_code: Option<String>,
    /// Whether search/partner network data was included.
    pub search_partners: Option<bool>,
    /// Competition level: "LOW", "MEDIUM" or "HIGH".
    pub competition: Option<String>,
    /// Competition as an index from 0 to 100.
    pub competition_index: Option<i32>,
    /// Average monthly search volume over the last 12 months.
    pub search_volume: Option<i64>,
    /// Lower range of the top-of-page bid, in USD.
    pub low_top_of_page_bid: Option<f64>,
    /// Upper range of the top-of-page bid, in USD.
    pub high_top_of_page_bid: Option<f64>,
    /// Average cost per click, in USD.
    pub cpc: Option<f64>,
    /// Search volume broken down by month.
    pub monthly_searches: Option<Vec<KeywordsDataApiGoogleAdsKeywordsForSiteTaskMonthlySearch>>,
    /// Concept groupings Google associates with the keyword.
    pub keyword_annotations: Option<KeywordsDataApiGoogleAdsKeywordsForSiteTaskKeywordAnnotations>,
}

/// A single month of historical search volume.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTaskMonthlySearch {
    /// Calendar year.
    pub year: Option<i32>,
    /// Calendar month (1-12).
    pub month: Option<i32>,
    /// Search volume for the month.
    pub search_volume: Option<i64>,
}

/// Concept annotations Google attaches to a keyword.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTaskKeywordAnnotations {
    /// Concepts associated with the keyword.
    pub concepts: Option<Vec<KeywordsDataApiGoogleAdsKeywordsForSiteTaskConcepts>>,
}

/// A single concept associated with a keyword.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTaskConcepts {
    /// Concept name.
    pub name: Option<String>,
    /// Group the concept belongs to.
    pub concept_group: Option<KeywordsDataApiGoogleAdsKeywordsForSiteTaskConceptGroup>,
}

/// The group a keyword concept belongs to.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTaskConceptGroup {
    /// Group name.
    pub name: Option<String>,
    /// Group type, e.g. "BRAND" or "NON_BRAND".
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
}
