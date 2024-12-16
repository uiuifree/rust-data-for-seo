use crate::entity::{
    SerpApiElementCarouselElement, SerpApiElementRelatedSearches, SerpApiHtmlItem,
    SerpApiRectangle, SerpApiTaskResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub type SerpApiGoogleImage<T> = SerpApiTaskResult<T>;
pub type SerpApiGoogleImagesAdvanced = SerpApiGoogleImage<SerpApiGoogleImagesItem>;
pub type SerpApiGoogleImagesHtml = SerpApiGoogleImage<SerpApiHtmlItem>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SerpApiGoogleImagesItem {
    #[serde(rename = "carousel")]
    Carousel(SerpApiGoogleImagesItemCarousel),
    #[serde(rename = "images_search")]
    TopStories(SerpApiGoogleImagesItemImageSearch),
    #[serde(rename = "related_searches")]
    RelatedSearches(SerpApiElementRelatedSearches),
    #[serde(untagged)]
    Unknown(Value),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleImagesItemCarousel {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub title: Option<String>,
    pub items: Option<Vec<SerpApiElementCarouselElement>>,
    pub rectangle: Option<SerpApiRectangle>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleImagesItemImageSearch {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub alt: Option<String>,
    pub url: Option<String>,
    pub source_url: Option<String>,
    pub encoded_url: Option<String>,
}
