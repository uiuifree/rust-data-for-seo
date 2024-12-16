use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAdsSearch {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub advertiser_id: Option<String>,
    pub creative_id: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub verified: Option<bool>,
    pub format: Option<String>,
    pub preview_image: Option<SerpApiElementAdsSearchPreviewImage>,
    pub preview_url: Option<String>,
    pub first_shown: Option<String>,
    pub last_shown: Option<String>,
}
/// preview image of the advertisement
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAdsSearchPreviewImage {
    pub url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
}
