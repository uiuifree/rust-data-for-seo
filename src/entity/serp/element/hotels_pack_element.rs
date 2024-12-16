use crate::entity::{SerpApiElementHotelsPackElementPrice, SerpApiRating};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementHotelsPackElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub price: Option<SerpApiElementHotelsPackElementPrice>,
    pub title: Option<String>,
    pub desription: Option<String>,
    pub hotel_identifier: Option<String>,
    pub domain: Option<String>,
    pub url: Option<String>,
    pub is_paid: Option<bool>,
    pub rating: Option<SerpApiRating>,
}
