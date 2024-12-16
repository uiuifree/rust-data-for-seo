use crate::entity::{SerpApiElementLinkElement, SerpApiElementRelatedResult, SerpApiGoogleOrganicItemPaidExtra, SerpApiPrice};
use serde::{Deserialize, Serialize};
use crate::entity::serp::element::images_element::SerpApiElementImagesElement;

/// Paid
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementPaid {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub title: Option<String>,
    pub domain: Option<String>,
    pub url: Option<String>,
    pub breadcrumb: Option<String>,
    pub is_image: Option<bool>,
    pub is_video: Option<bool>,
    pub images: Option<Vec<SerpApiElementImagesElement>>,
    pub highlighted: Option<Vec<String>>,
    pub extra: Option<SerpApiGoogleOrganicItemPaidExtra>,
    pub description: Option<String>,
    pub description_rows: Option<String>,
    pub links: Option<Vec<SerpApiElementLinkElement>>,
    pub price: Option<SerpApiPrice>,
    pub related_result: Option<SerpApiElementRelatedResult>,
}
