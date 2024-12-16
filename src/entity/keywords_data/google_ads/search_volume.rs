


use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsSearchVolumeTask {
    pub keyword: Option<String>,
    pub spell: Option<String>,
    pub location_code: Option<u32>,
    pub language_code: Option<String>,
    pub search_partners: Option<bool>,
    pub competition: Option<String>,
    pub competition_index: Option<i32>,
    pub search_volume: Option<i32>,
    pub low_top_of_page_bid: Option<f32>,
    pub high_top_of_page_bid: Option<f32>,
    pub cpc: Option<f32>,
    pub monthly_searches: Option<Vec<KeywordsDataApiGoogleAdsSearchVolumeTaskMonthlySearch>>,
}


#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct KeywordsDataApiGoogleAdsSearchVolumeTaskMonthlySearch{
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub search_volume: Option<i32>,

}