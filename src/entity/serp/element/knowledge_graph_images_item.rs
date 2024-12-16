use crate::entity::serp::element::knowledge_graph_images_element::SerpApiElementKnowledgeGraphImagesElement;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementKnowledgeGraphImagesItem {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub card_id: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub logo_url: Option<String>,
    pub cid: Option<String>,

    pub items: Option<Vec<SerpApiElementKnowledgeGraphImagesElement>>,
}
