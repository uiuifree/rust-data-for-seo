use crate::entity::{SerpApiRating, SerpApiRatingDistribution};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementMapsSearch {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub contact_url: Option<String>,
    pub contributor_url: Option<String>,
    pub book_online_url: Option<String>,
    pub rating: Option<SerpApiRating>,
    pub rating_distribution: Option<SerpApiRatingDistribution>,
}
