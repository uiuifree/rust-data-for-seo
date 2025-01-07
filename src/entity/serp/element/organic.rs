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
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub cache_url: Option<String>,
    pub related_search_url: Option<String>,
    pub breadcrumb: Option<String>,
    pub website_name: Option<String>,
    pub is_image: Option<bool>,
    pub is_video: Option<bool>,
    pub is_featured_snippet: Option<bool>,
    pub is_malicious: Option<bool>,
    pub is_web_story: Option<bool>,
    pub description: Option<String>,
    pub pre_snippet: Option<String>,
    pub extended_snippet: Option<String>,
    pub images: Option<Vec<SerpApiElementImagesElement>>,
    pub amp_version: Option<bool>,
    pub rating: Option<SerpApiRating>,
    pub price: Option<SerpApiPrice>,
    pub highlighted: Option<Vec<String>>,
    pub links: Option<Vec<SerpApiElementLinkElement>>,
    pub faq: Option<SerpApiElementFaqBox>,
    pub extended_people_also_search: Option<Vec<Value>>,
    pub about_this_result: Option<SerpApiElementAboutThisResultElement>,
    pub related_result: Option<SerpApiElementRelatedResult>,
    pub timestamp: Option<String>,
    pub rectangle: Option<SerpApiRectangle>,
}
