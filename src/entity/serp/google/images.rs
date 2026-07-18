use crate::entity::{
    SerpApiElementCarouselElement, SerpApiElementRelatedSearches, SerpApiHtmlItem,
    SerpApiRectangle, SerpApiTaskResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Image result type.
pub type SerpApiGoogleImage<T> = SerpApiTaskResult<T>;
/// Google Images Advanced result type.
pub type SerpApiGoogleImagesAdvanced = SerpApiGoogleImage<SerpApiGoogleImagesItem>;
/// Google Images Html result type.
pub type SerpApiGoogleImagesHtml = SerpApiGoogleImage<SerpApiHtmlItem>;

/// Google Images Item: a SERP element tagged by the DataForSEO `type` field.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SerpApiGoogleImagesItem {
    /// Element of type `carousel`.
    #[serde(rename = "carousel")]
    Carousel(SerpApiGoogleImagesItemCarousel),
    /// Element of type `top_stories`.
    #[serde(rename = "images_search")]
    TopStories(SerpApiGoogleImagesItemImageSearch),
    /// Element of type `related_searches`.
    #[serde(rename = "related_searches")]
    RelatedSearches(SerpApiElementRelatedSearches),
    /// Fallback holding the raw JSON of an unrecognized `type`.
    #[serde(untagged)]
    Unknown(Value),
}
/// Google Images Item Carousel SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleImagesItemCarousel {
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Alignment of the element within the SERP, `left` or `right`.
    pub position: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiElementCarouselElement>>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
/// Google Images Item Image Search SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleImagesItemImageSearch {
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Alignment of the element within the SERP, `left` or `right`.
    pub position: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Subtitle text of the result.
    pub subtitle: Option<String>,
    /// Alternative text of the image.
    pub alt: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// URL of the source that published the result.
    pub source_url: Option<String>,
    /// Encoded URL of the image resource.
    pub encoded_url: Option<String>,
}
