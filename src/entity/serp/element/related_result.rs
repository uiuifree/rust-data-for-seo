use crate::entity::serp::element::images_element::SerpApiElementImagesElement;
use crate::entity::{SerpApiElementAboutThisResultElement, SerpApiPrice, SerpApiRating};
use serde::{Deserialize, Serialize};

/// Related Result SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementRelatedResult {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
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
    /// `true` if the result contains an image.
    pub is_image: Option<bool>,
    /// `true` if the result contains a video.
    pub is_video: Option<bool>,
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
    /// "About this result" panel data.
    pub about_this_result: Option<Vec<SerpApiElementAboutThisResultElement>>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
}
