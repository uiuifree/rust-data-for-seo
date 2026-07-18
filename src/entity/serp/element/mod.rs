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
mod images_element;
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
mod organic;
mod paid;
mod people_also_ask;
mod people_also_ask_element;
mod people_also_ask_extended_element;
mod people_also_search;
mod refinement_chips;
mod refinement_chips_element;
mod refinement_chips_option;
mod related_result;
mod related_searches;

pub use about_this_result_element::*;
pub use ads_advertiser::*;
pub use answer_box::*;
// pub use app::*;
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
pub use featured_snippet::*;
pub use hotels_pack::*;
pub use images_element::*;
pub use knowledge_graph::*;
pub use knowledge_graph_ai_overview_item::*;
pub use knowledge_graph_images_element::*;
pub use knowledge_graph_images_item::*;
pub use link_element::*;
pub use local_pack::*;
pub use map_search::*;
pub use multi_carousel::*;
pub use multi_carousel_element::*;
pub use multi_carousel_snippet::*;
pub use organic::*;
pub use paid::*;
pub use people_also_ask::*;
pub use people_also_ask_element::*;
pub use people_also_ask_extended_element::*;
pub use people_also_search::*;
pub use refinement_chips::*;
pub use refinement_chips_element::*;
pub use refinement_chips_option::*;
pub use related_result::*;
pub use related_searches::*;

use crate::entity::{SerpApiElementTopStoriesElement, SerpApiRectangle};
use serde::{Deserialize, Serialize};

/// GoogleJobsItem
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementGoogleNewsSearch {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// URL of the image.
    pub image_url: Option<String>,
    /// Snippet text of the result.
    pub snippet: Option<String>,
    /// Time published.
    pub time_published: Option<String>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
/// TopStories
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleOrganicItemTopStories {
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Alignment of the element within the SERP, `left` or `right`.
    pub position: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Parsed elements of the result.
    pub items: Option<SerpApiElementTopStoriesElement>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
/// EventItem
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementEventItem {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Alignment of the element within the SERP, `left` or `right`.
    pub position: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// URL of the image.
    pub image_url: Option<String>,
    /// Dates the event takes place.
    pub event_dates: Option<Vec<SerpApiElementEventItemEventDate>>,
    /// Venue and location details of the event.
    pub location_info: Option<SerpApiElementEventItemLocationInfo>,
    /// Links to event information and tickets.
    pub information_and_tickets: Option<Vec<SerpApiElementInformationAndTicketsElement>>,
}
/// EventItemEventDate
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementEventItemEventDate {
    /// Start date and time of the event.
    pub start_datetime: Option<String>,
    /// End date and time of the event.
    pub end_datetime: Option<String>,
    /// Event dates as displayed in the SERP.
    pub displayed_dates: Option<String>,
}
/// EventItemLocationInfo
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementEventItemLocationInfo {
    /// Name of the entity.
    pub name: Option<String>,
    /// Postal address of the entity.
    pub address: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Google CID identifier of the entity.
    pub cid: Option<String>,
    /// Google feature identifier of the entity.
    pub feature_id: Option<String>,
}
/// information_and_tickets_element
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementInformationAndTicketsElement {
    /// Postal address of the entity.
    #[serde(rename = "type")]
    pub address: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
}
/// GoogleJobsItem
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementGoogleJobsItem {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Alignment of the element within the SERP, `left` or `right`.
    pub position: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Identifier of the job posting.
    pub job_id: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Name of the employer.
    pub employer_name: Option<String>,
    /// URL of the employer.
    pub employer_url: Option<String>,
    /// URL of the employer's logo.
    pub employer_image_url: Option<String>,
    /// Location of the entity.
    pub location: Option<String>,
    /// Name of the source that published the result.
    pub source_name: Option<String>,
    /// URL of the source that published the result.
    pub source_url: Option<String>,
    /// Salary shown for the job.
    pub salary: Option<String>,
    /// Employment contract type.
    pub contract_type: Option<String>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// Pixel bounding box of the element (when `calculate_rectangles` is set).
    pub rectangle: Option<SerpApiRectangle>,
}
/// Autocomplete
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAutocomplete {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Relevance score of the suggestion.
    pub relevance: Option<i32>,
    /// Suggested search query.
    pub suggestion: Option<String>,
    /// Type of the autocomplete suggestion.
    pub suggestion_type: Option<String>,
    /// URL that runs the suggested query.
    pub search_query_url: Option<String>,
    /// URL of the thumbnail image.
    pub thumbnail_url: Option<String>,
    /// Substrings the search engine highlighted in the result.
    pub highlighted: Option<Vec<String>>,
}
/// Dataset
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDataset {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Rank of the element among elements of the same type.
    pub rank_group: Option<i32>,
    /// Absolute rank of the element across the whole SERP.
    pub rank_absolute: Option<i32>,
    /// Alignment of the element within the SERP, `left` or `right`.
    pub position: Option<String>,
    /// XPath of the element within the page.
    pub xpath: Option<String>,
    /// Google Dataset Search identifier.
    pub dataset_id: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the image.
    pub image_url: Option<String>,
    /// Number of scholarly citations.
    pub scholarly_citations_count: Option<i32>,
    /// URL listing scholarly articles.
    pub scholarly_articles_url: Option<String>,
    /// Unique identifier of the dataset.
    pub unique_identifier: Option<String>,
    /// Related article reference.
    pub related_article: Option<String>,
    /// Links associated with the result.
    pub links: Option<Vec<SerpApiElementLinkElement>>,
    /// Providers that publish the dataset.
    pub dataset_providers: Option<Vec<SerpApiElementDatasetProvidersElement>>,
    /// Available file formats of the dataset.
    pub formats: Option<Vec<SerpApiElementFormatsElement>>,
    /// Authors of the dataset.
    pub authors: Option<Vec<SerpApiElementAuthorsElement>>,
    /// Licenses the dataset is released under.
    pub licenses: Option<Vec<SerpApiElementLicencesElement>>,
    /// Date the dataset was last updated.
    pub updated_date: Option<String>,
    /// Geographic areas the dataset covers.
    pub area_covered: Option<Vec<String>>,
    /// Time period the dataset covers.
    pub period_covered: Option<SerpApiElementDatasetPeriodCovered>,
    /// Description of the dataset.
    pub dataset_description: Option<SerpApiElementDatasetDescription>,
}

/// Dataset Period Covered SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDatasetPeriodCovered {
    start_date: Option<String>,
    end_date: Option<String>,
    displayed_date: Option<String>,
}
/// Dataset Description SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDatasetDescription {
    text: Option<String>,
    links: Option<Vec<SerpApiElementLinkElement>>,
}

/// Dataset Providers Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementDatasetProvidersElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
}
/// Formats Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementFormatsElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Ad creative format.
    pub format: Option<String>,
    /// Size of the file.
    pub size: Option<String>,
}
/// Authors Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementAuthorsElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Name of the entity.
    pub name: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
}
/// Licences Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementLicencesElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
}
