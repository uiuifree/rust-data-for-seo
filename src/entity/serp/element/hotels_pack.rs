use crate::entity::serp::element::hotels_pack_element::SerpApiElementHotelsPackElement;
use crate::entity::SerpApiRectangle;
use serde::{Deserialize, Serialize};

/// HotelsPack
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementHotelsPack {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub title: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub items: Option<SerpApiElementHotelsPackElement>,
    pub rectangle: Option<SerpApiRectangle>,
}
