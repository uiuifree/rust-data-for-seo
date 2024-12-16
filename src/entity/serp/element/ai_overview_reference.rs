use serde::{Deserialize, Serialize};
use crate::entity::serp::element::images_element::SerpApiElementImagesElement;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAiOverviewReference {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,

    pub source: Option<String>,
    pub domain: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub text: Option<String>,
    pub images: Option<Vec<SerpApiElementImagesElement>>,
}
