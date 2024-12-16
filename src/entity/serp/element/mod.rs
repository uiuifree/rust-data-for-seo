mod about_this_result_element;
mod ads_advertiser;
mod ads_domain;
mod ads_multi_account_advertiser;
mod ads_search;
mod advertiser;
mod ai_overview;
mod ai_overview_element;
mod ai_overview_reference;
mod answer_box;
mod carousel;
mod carousel_element;
mod compare_sites;
mod compare_sites_element;
mod discussions_and_forums;
mod discussions_and_forums_element;
mod faq_box;
mod faq_box_element;
mod featured_snippet;
mod hotels_pack;
mod hotels_pack_element;
mod knowledge_graph;
mod knowledge_graph_ai_overview_item;
mod knowledge_graph_images_element;
mod knowledge_graph_images_item;
mod link_element;
mod local_pack;
mod map_search;
mod multi_carousel;
mod multi_carousel_element;
mod multi_carousel_snippet;
mod paid;
mod people_also_search;
mod refinement_chips;
mod refinement_chips_element;
mod refinement_chips_option;
mod related_result;
mod related_searches;
mod organic;
mod images_element;

pub use about_this_result_element::*;
pub use ads_advertiser::*;
pub use ads_domain::*;
pub use ads_multi_account_advertiser::*;
pub use ads_search::*;
pub use advertiser::*;
pub use ai_overview::*;
pub use ai_overview_element::*;
pub use ai_overview_reference::*;
pub use carousel::*;
pub use carousel_element::*;
pub use compare_sites::*;
pub use compare_sites_element::*;
pub use discussions_and_forums::*;
pub use discussions_and_forums_element::*;
pub use faq_box::*;
pub use faq_box_element::*;
pub use link_element::*;
pub use local_pack::*;
pub use multi_carousel::*;
pub use multi_carousel_element::*;
pub use multi_carousel_snippet::*;
pub use paid::*;
pub use images_element::*;
pub use refinement_chips::*;
pub use refinement_chips_element::*;
pub use refinement_chips_option::*;
pub use related_result::*;
pub use related_searches::*;
pub use organic::*;

use crate::entity::{SerpApiElementTopStoriesElement, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// GoogleJobsItem
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementGoogleNewsSearch {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub xpath: Option<String>,
    pub domain: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub snippet: Option<String>,
    pub time_published: Option<String>,
    pub timestamp: Option<String>,
    pub rectangle: Option<SerpApiRectangle>,
}
/// TopStories
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleOrganicItemTopStories {
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub title: Option<String>,
    pub items: Option<SerpApiElementTopStoriesElement>,
    pub rectangle: Option<SerpApiRectangle>,
}
/// EventItem
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementEventItem {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub event_dates: Option<Vec<SerpApiElementEventItemEventDate>>,
    pub location_info: Option<SerpApiElementEventItemLocationInfo>,
    pub information_and_tickets: Option<Vec<SerpApiElementInformationAndTicketsElement>>,
}
/// EventItemEventDate
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementEventItemEventDate {
    pub start_datetime: Option<String>,
    pub end_datetime: Option<String>,
    pub displayed_dates: Option<String>,
}
/// EventItemLocationInfo
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementEventItemLocationInfo {
    pub name: Option<String>,
    pub address: Option<String>,
    pub url: Option<String>,
    pub cid: Option<String>,
    pub feature_id: Option<String>,
}
/// information_and_tickets_element
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementInformationAndTicketsElement {
    #[serde(rename = "type")]
    pub address: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
}
/// GoogleJobsItem
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementGoogleJobsItem {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub job_id: Option<String>,
    pub title: Option<String>,
    pub employer_name: Option<String>,
    pub employer_url: Option<String>,
    pub employer_image_url: Option<String>,
    pub location: Option<String>,
    pub source_name: Option<String>,
    pub source_url: Option<String>,
    pub salary: Option<String>,
    pub contract_type: Option<String>,
    pub timestamp: Option<String>,
    pub rectangle: Option<SerpApiRectangle>,
}
/// Autocomplete
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAutocomplete {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub relevance: Option<i32>,
    pub suggestion: Option<String>,
    pub suggestion_type: Option<String>,
    pub search_query_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub highlighted: Option<Vec<String>>,
}
/// Dataset
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDataset {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub rank_group: Option<i32>,
    pub rank_absolute: Option<i32>,
    pub position: Option<String>,
    pub xpath: Option<String>,
    pub dataset_id: Option<String>,
    pub title: Option<String>,
    pub image_url: Option<String>,
    pub scholarly_citations_count: Option<i32>,
    pub scholarly_articles_url: Option<String>,
    pub unique_identifier: Option<String>,
    pub related_article: Option<String>,
    pub links: Option<Vec<SerpApiElementLinkElement>>,
    pub dataset_providers: Option<Vec<SerpApiElementDatasetProvidersElement>>,
    pub formats: Option<Vec<SerpApiElementFormatsElement>>,
    pub authors: Option<Vec<SerpApiElementAuthorsElement>>,
    pub licenses: Option<Vec<SerpApiElementLicencesElement>>,
    pub updated_date: Option<String>,
    pub area_covered: Option<Vec<String>>,
    pub period_covered: Option<SerpApiElementDatasetPeriodCovered>,
    pub dataset_description: Option<SerpApiElementDatasetDescription>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDatasetPeriodCovered {
    start_date: Option<String>,
    end_date: Option<String>,
    displayed_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDatasetDescription {
    text: Option<String>,
    links: Option<Vec<SerpApiElementLinkElement>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDatasetProvidersElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFormatsElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub format: Option<String>,
    pub size: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAuthorsElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementLicencesElement {
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub domain: Option<String>,
}
