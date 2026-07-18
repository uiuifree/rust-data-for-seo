use crate::entity::serp::element::knowledge_graph_images_element::SerpApiElementKnowledgeGraphImagesElement;
use serde::{Deserialize, Serialize};

/// Knowledge Graph Images Item SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementKnowledgeGraphImagesItem {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
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
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// Identifier of the knowledge-graph card.
    pub card_id: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// URL of the image.
    pub image_url: Option<String>,
    /// URL of the logo image.
    pub logo_url: Option<String>,
    /// Google CID identifier of the entity.
    pub cid: Option<String>,

    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiElementKnowledgeGraphImagesElement>>,
}
