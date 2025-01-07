use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTask {
    pub keyword: Option<String>,
    // pub spell: Option<String>,
    pub location_code: Option<u32>,
    pub language_code: Option<String>,
    pub search_partners: Option<bool>,
    pub competition: Option<String>,
    pub competition_index: Option<i32>,
    pub search_volume: Option<i32>,
    pub low_top_of_page_bid: Option<f32>,
    pub high_top_of_page_bid: Option<f32>,
    pub cpc: Option<f32>,
    pub monthly_searches: Option<Vec<KeywordsDataApiGoogleAdsKeywordsForSiteTaskMonthlySearch>>,
    pub keyword_annotations: Option<KeywordsDataApiGoogleAdsKeywordsForSiteTaskKeywordAnnotations>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTaskMonthlySearch {
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub search_volume: Option<i32>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTaskKeywordAnnotations {
    pub concepts: Option<Vec<KeywordsDataApiGoogleAdsKeywordsForSiteTaskConcepts>>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTaskConcepts {
    pub name: Option<String>,
    pub concept_group: Option<KeywordsDataApiGoogleAdsKeywordsForSiteTaskConceptGroup>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsKeywordsForSiteTaskConceptGroup {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub concept_type: Option<String>,
}
