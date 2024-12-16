use crate::entity::{SerpApiElementCarousel, SerpApiElementOrganic, SerpApiElementPaid, SerpApiElementRefinementChips, SerpApiGoogleOrganicTaskSpell};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleOrganicTaskRegular {
    pub keyword: String,
    #[serde(rename = "type")]
    pub search_engine_type: Option<String>,
    pub se_domain: Option<String>,
    pub location_code: i32,
    pub language_code: Option<String>,
    pub check_url: Option<String>,
    pub datetime: Option<String>,
    pub spell: Option<SerpApiGoogleOrganicTaskSpell>,
    pub refinement_chips: Option<SerpApiElementRefinementChips>,
    pub item_types: Option<Vec<String>>,
    pub se_results_count: Option<i32>,
    pub items_count: Option<i32>,
    // pub items: Option<Vec<Value>>,
    pub items: Option<Vec<SerpApiGoogleOrganicItem>>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleOrganicTaskAdvanced {
    pub keyword: String,
    #[serde(rename = "type")]
    pub search_engine_type: Option<String>,
    pub se_domain: Option<String>,
    pub location_code: i32,
    pub language_code: Option<String>,
    pub check_url: Option<String>,
    pub datetime: Option<String>,
    pub spell: Option<SerpApiGoogleOrganicTaskSpell>,
    pub refinement_chips: Option<SerpApiElementRefinementChips>,
    pub item_types: Option<Vec<String>>,
    pub se_results_count: Option<i32>,
    pub items_count: Option<i32>,
    // pub items: Option<Vec<Value>>,
    pub items: Option<Vec<SerpApiGoogleOrganicItem>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum SerpApiGoogleOrganicItem {
    #[serde(rename = "organic")]
    Organic(SerpApiElementOrganic),
    #[serde(rename = "paid")]
    Paid(SerpApiElementPaid),
    #[serde(rename = "carousel")]
    Carousel(SerpApiElementCarousel),
    #[serde(untagged)]
    Unknown(Value),
}

// answer_box
// app
// carousel
// multi_carousel
// featured_snippet
// google_flights
// google_reviews
// google_posts
// images
// jobs
// knowledge_graph
// local_pack
// hotels_pack
// map
// organic
// paid
// people_also_ask
// related_searches
// people_also_search
// shopping
// top_stories
// twitter
// video
// events
// mention_carousel
// recipes
// top_sights
// scholarly_articles
// popular_products
// podcasts
// questions_and_answers
// find_results_on
// stocks_box
// visual_stories
// commercial_units
// local_services
// google_hotels
// math_solver
// currency_box
// product_considerations
// found_on_web
// short_videos
// refine_products
// explore_brands
// perspectives
// discussions_and_forums
// compare_sites
// courses
// ai_overview

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleOrganicItemPaidExtra {
    ad_aclk: Option<String>,
}

// --- item --------------------------------------------//



// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct SerpApiElementFaq {
//     #[serde(rename = "type")]
//     pub type_of_element: Option<String>,
//     pub title: Option<String>,
//     pub description: Option<String>,
//     pub url: Option<String>,
//     pub domain: Option<String>,
// }


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementHotelsPackElementPrice {
    pub current: Option<f32>,
    pub regular: Option<f32>,
    pub max_value: Option<f32>,
    pub currency: Option<String>,
    pub is_price_range: Option<bool>,
    pub displayed_price: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementTopStoriesElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub source: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub date: Option<String>,
    pub amp_version: Option<bool>,
    pub timestamp: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub badges: Option<Vec<String>>,
}
