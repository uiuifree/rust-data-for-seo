use crate::entity::{SerpApiElementCarouselElement, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// Carousel
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementCarousel {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub title: Option<String>,
    pub items: Option<Vec<SerpApiElementCarouselElement>>,
    pub rectangle: Option<SerpApiRectangle>,
}
