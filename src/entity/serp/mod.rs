mod element;
mod google;
mod languages;

pub use element::*;
pub use google::*;
pub use languages::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiHtmlItem {
    pub page: i32,
    pub date: String,
    pub html: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiRating {
    pub rating_type: Option<String>,
    pub value: Option<f32>,
    pub votes_count: Option<u32>,
    pub rating_max: Option<u32>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiRatingDistribution {
    #[serde(rename = "1")]
    pub distribution1: Option<i32>,
    #[serde(rename = "2")]
    pub distribution2: Option<i32>,
    #[serde(rename = "3")]
    pub distribution3: Option<i32>,
    #[serde(rename = "4")]
    pub distribution4: Option<i32>,
    #[serde(rename = "5")]
    pub distribution5: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiRectangle {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiPrice {
    pub current: Option<f32>,
    pub regular: Option<f32>,
    pub max_value: Option<f32>,
    pub currency: Option<String>,
    pub is_price_range: Option<bool>,
    pub displayed_price: Option<String>,
}