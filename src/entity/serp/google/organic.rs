use crate::entity::{
    SerpApiElementAiOverview, SerpApiElementAnswerBox, SerpApiElementCarousel,
    SerpApiElementFeaturedSnippet, SerpApiElementHotelsPack, SerpApiElementKnowledgeGraph,
    SerpApiElementLocalPack, SerpApiElementMultiCarousel, SerpApiElementOrganic,
    SerpApiElementPaid, SerpApiElementPeopleAlsoAsk, SerpApiElementPeopleAlsoSearch,
    SerpApiElementRefinementChips, SerpApiElementRelatedSearches, SerpApiGoogleOrganicTaskSpell,
};
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
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SerpApiGoogleOrganicItem {
    #[serde(rename = "answer_box")]
    AnswerBox(SerpApiElementAnswerBox),
    // #[serde(rename="app")]
    // App(SerpApiElementApp),
    #[serde(rename = "carousel")]
    Carousel(SerpApiElementCarousel),
    #[serde(rename = "multi_carousel")]
    MultiCarousel(SerpApiElementMultiCarousel),
    #[serde(rename = "featured_snippet")]
    FeaturedSnippet(SerpApiElementFeaturedSnippet),
    // #[serde(rename="google_flights")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="google_reviews")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="google_posts")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="images")]
    // Organic(SerpApiElementImages),
    // #[serde(rename="jobs")]
    // Organic(SerpApiElementOrganic),
    #[serde(rename = "knowledge_graph")]
    KnowledgeGraph(SerpApiElementKnowledgeGraph),
    #[serde(rename = "local_pack")]
    LocalPack(SerpApiElementLocalPack),
    #[serde(rename = "hotels_pack")]
    HotelsPack(SerpApiElementHotelsPack),
    // #[serde(rename="map")]
    // Organic(SerpApiElementMap),
    #[serde(rename = "people_also_ask")]
    PeopleAlsoAsk(SerpApiElementPeopleAlsoAsk),

    #[serde(rename = "organic")]
    Organic(SerpApiElementOrganic),
    #[serde(rename = "paid")]
    Paid(SerpApiElementPaid),
    #[serde(rename = "related_searches")]
    RelatedSearches(SerpApiElementRelatedSearches),

    #[serde(rename = "people_also_search")]
    PeopleAlsoSearch(SerpApiElementPeopleAlsoSearch),
    // #[serde(rename="shopping")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="top_stories")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="twitter")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="video")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="events")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="mention_carousel")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="recipes")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="top_sights")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="scholarly_articles")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="popular_products")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="podcasts")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="questions_and_answers")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="find_results_on")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="stocks_box")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="visual_stories")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="commercial_units")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="local_services")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="google_hotels")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="math_solver")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="currency_box")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="product_considerations")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="found_on_web")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="short_videos")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="refine_products")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="explore_brands")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="perspectives")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="discussions_and_forums")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="compare_sites")]
    // Organic(SerpApiElementOrganic),
    // #[serde(rename="courses")]
    // Organic(SerpApiElementOrganic),
    #[serde(rename = "ai_overview")]
    AiOverview(SerpApiElementAiOverview),

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
//
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
