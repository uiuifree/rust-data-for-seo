use crate::entity::serp::element::images_element::SerpApiElementImagesElement;
use crate::entity::{SerpApiElementAiOverviewReference, SerpApiRectangle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAiOverviewElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,

    pub title: Option<String>,
    pub text: Option<String>,
    pub images: Option<Vec<SerpApiElementImagesElement>>,
    pub references: Option<Vec<SerpApiElementAiOverviewReference>>,
    pub rectangle: Option<SerpApiRectangle>,
}
