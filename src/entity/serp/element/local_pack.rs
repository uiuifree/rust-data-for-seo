use crate::entity::{SerpApiRating, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// LocalPack
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementLocalPack {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub domain: Option<String>,
    pub phone: Option<String>,
    pub url: Option<String>,
    pub is_paid: Option<String>,
    pub rating: Option<SerpApiRating>,
    pub cid: Option<String>,
    pub rectangle: Option<SerpApiRectangle>,
}
