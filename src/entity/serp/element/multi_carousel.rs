use crate::entity::{SerpApiElementMultiCarouselElement, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// MultiCarousel
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementMultiCarousel {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub items: Option<Vec<SerpApiElementMultiCarouselElement>>,
    pub rectangle: Option<SerpApiRectangle>,
}
