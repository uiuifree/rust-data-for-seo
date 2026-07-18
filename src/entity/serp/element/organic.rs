use crate::entity::serp::element::images_element::SerpApiElementImagesElement;
use crate::entity::{
    SerpApiElementAboutThisResultElement, SerpApiElementFaqBox, SerpApiElementLinkElement,
    SerpApiElementRelatedResult, SerpApiPrice, SerpApiRating, SerpApiRectangle,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Organic
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementOrganic {
    // #[serde(rename="type")]
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i64>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i64>,
    /// Alignment of the element within the SERP, `left` or `right`.
    pub position: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// URL of the cached page, if available.
    pub cache_url: Option<String>,
    /// URL of the related search, if any.
    pub related_search_url: Option<String>,
    /// Breadcrumb shown under the result URL.
    pub breadcrumb: Option<String>,
    /// Displayed website name of the result.
    pub website_name: Option<String>,
    /// `true` if the result contains an image.
    pub is_image: Option<bool>,
    /// `true` if the result contains a video.
    pub is_video: Option<bool>,
    /// `true` if the result is a featured snippet.
    pub is_featured_snippet: Option<bool>,
    /// `true` if the result is flagged as malicious.
    pub is_malicious: Option<bool>,
    /// `true` if the result is a Google Web Story.
    pub is_web_story: Option<bool>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// Text shown before the main snippet.
    pub pre_snippet: Option<String>,
    /// Additional snippet text shown for the result.
    pub extended_snippet: Option<String>,
    /// Images attached to the result.
    pub images: Option<Vec<SerpApiElementImagesElement>>,
    /// `true` if an AMP version of the page is available.
    pub amp_version: Option<bool>,
    /// Aggregate rating shown for the result.
    pub rating: Option<SerpApiRating>,
    /// Price information shown for the result.
    pub price: Option<SerpApiPrice>,
    /// Substrings the search engine highlighted in the result.
    pub highlighted: Option<Vec<String>>,
    /// Links associated with the result.
    pub links: Option<Vec<SerpApiElementLinkElement>>,
    /// FAQ block attached to the result.
    pub faq: Option<SerpApiElementFaqBox>,
    /// Extended people also search.
    pub extended_people_also_search: Option<Vec<Value>>,
    /// "About this result" panel data.
    pub about_this_result: Option<SerpApiElementAboutThisResultElement>,
    /// Related result shown alongside this one.
    pub related_result: Option<SerpApiElementRelatedResult>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
