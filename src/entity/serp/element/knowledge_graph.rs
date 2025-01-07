use crate::entity::serp::element::hotels_pack_element::SerpApiElementHotelsPackElement;
use crate::entity::SerpApiRectangle;
use serde::{Deserialize, Serialize};

/// KnowledgeGraph
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementKnowledgeGraph {
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
    pub items: Option<Vec<SerpApiElementHotelsPackElement>>,
    pub rectangle: Option<SerpApiRectangle>,
}
