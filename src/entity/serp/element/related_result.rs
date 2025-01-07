use crate::entity::serp::element::images_element::SerpApiElementImagesElement;
use crate::entity::{SerpApiElementAboutThisResultElement, SerpApiPrice, SerpApiRating};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementRelatedResult {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub xpath: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub cache_url: Option<String>,
    pub related_search_url: Option<String>,
    pub breadcrumb: Option<String>,
    pub is_image: Option<bool>,
    pub is_video: Option<bool>,
    pub description: Option<String>,
    pub pre_snippet: Option<String>,
    pub extended_snippet: Option<String>,
    pub images: Option<Vec<SerpApiElementImagesElement>>,
    pub amp_version: Option<bool>,
    pub rating: Option<SerpApiRating>,
    pub price: Option<SerpApiPrice>,
    pub highlighted: Option<Vec<String>>,
    pub about_this_result: Option<Vec<SerpApiElementAboutThisResultElement>>,
    pub timestamp: Option<String>,
}
