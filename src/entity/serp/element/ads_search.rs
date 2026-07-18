use serde::{Deserialize, Serialize};

/// Ads Search SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAdsSearch {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i64>,
    /// Google Ads advertiser identifier.
    pub advertiser_id: Option<String>,
    /// Google Ads creative identifier.
    pub creative_id: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// `true` if the advertiser is verified.
    pub verified: Option<bool>,
    /// Ad creative format.
    pub format: Option<String>,
    /// Preview image of the advertisement.
    pub preview_image: Option<SerpApiElementAdsSearchPreviewImage>,
    /// URL to preview the advertisement.
    pub preview_url: Option<String>,
    /// Timestamp the ad was first shown.
    pub first_shown: Option<String>,
    /// Timestamp the ad was last shown.
    pub last_shown: Option<String>,
}
/// preview image of the advertisement
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAdsSearchPreviewImage {
    /// URL of the result.
    pub url: Option<String>,
    /// Height of the bounding box, in pixels.
    pub height: Option<i64>,
    /// Width of the bounding box, in pixels.
    pub width: Option<i64>,
}
