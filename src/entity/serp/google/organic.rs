use crate::entity::{
    SerpApiElementAiOverview, SerpApiElementAnswerBox, SerpApiElementCarousel,
    SerpApiElementFeaturedSnippet, SerpApiElementHotelsPack, SerpApiElementKnowledgeGraph,
    SerpApiElementLocalPack, SerpApiElementMultiCarousel, SerpApiElementOrganic,
    SerpApiElementPaid, SerpApiElementPeopleAlsoAsk, SerpApiElementPeopleAlsoSearch,
    SerpApiElementRefinementChips, SerpApiElementRelatedSearches, SerpApiGoogleOrganicTaskSpell,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Google Organic Task Regular SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleOrganicTaskRegular {
    /// Search term the result was returned for.
    pub keyword: String,
    /// Search engine result type (the API `type` field).
    #[serde(rename = "type")]
    pub search_engine_type: Option<String>,
    /// Search-engine domain the results were taken from.
    pub se_domain: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: i32,
    /// Language code the search was run for.
    pub language_code: Option<String>,
    /// Direct URL to reproduce the search on the search engine.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was received.
    pub datetime: Option<String>,
    /// Search-engine spelling correction applied to the query, if any.
    pub spell: Option<SerpApiGoogleOrganicTaskSpell>,
    /// Search-refinement chips shown for the query.
    pub refinement_chips: Option<SerpApiElementRefinementChips>,
    /// Distinct element types present in the returned SERP.
    pub item_types: Option<Vec<String>>,
    /// Total number of results reported by the search engine.
    pub se_results_count: Option<i64>,
    /// Number of items returned in this result.
    pub items_count: Option<i64>,
    // pub items: Option<Vec<Value>>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiGoogleOrganicItem>>,
}
/// Google Organic Task Advanced SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiGoogleOrganicTaskAdvanced {
    /// Search term the result was returned for.
    pub keyword: String,
    /// Search engine result type (the API `type` field).
    #[serde(rename = "type")]
    pub search_engine_type: Option<String>,
    /// Search-engine domain the results were taken from.
    pub se_domain: Option<String>,
    /// DataForSEO location code the search was run for.
    pub location_code: i32,
    /// Language code the search was run for.
    pub language_code: Option<String>,
    /// Direct URL to reproduce the search on the search engine.
    pub check_url: Option<String>,
    /// UTC timestamp when the result was received.
    pub datetime: Option<String>,
    /// Search-engine spelling correction applied to the query, if any.
    pub spell: Option<SerpApiGoogleOrganicTaskSpell>,
    /// Search-refinement chips shown for the query.
    pub refinement_chips: Option<SerpApiElementRefinementChips>,
    /// Distinct element types present in the returned SERP.
    pub item_types: Option<Vec<String>>,
    /// Total number of results reported by the search engine.
    pub se_results_count: Option<i64>,
    /// Number of items returned in this result.
    pub items_count: Option<i64>,
    // pub items: Option<Vec<Value>>,
    /// Parsed elements of the result.
    pub items: Option<Vec<SerpApiGoogleOrganicItem>>,
}

impl SerpApiGoogleOrganicTaskAdvanced {
    /// Items of this result (empty slice when the API returned none).
    pub fn items(&self) -> &[SerpApiGoogleOrganicItem] {
        self.items.as_deref().unwrap_or_default()
    }
}

/// A single item in a Google organic SERP, tagged by the DataForSEO `type` field.
///
/// Large variants are boxed to keep the enum small (avoids `clippy::large_enum_variant`).
/// Unrecognized `type` values fall back to [`SerpApiGoogleOrganicItem::Unknown`].
/// See <https://docs.dataforseo.com/v3/serp/google/organic/task_get/advanced/>.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SerpApiGoogleOrganicItem {
    /// Element of type `answer_box`.
    #[serde(rename = "answer_box")]
    AnswerBox(Box<SerpApiElementAnswerBox>),
    /// Element of type `carousel`.
    #[serde(rename = "carousel")]
    Carousel(Box<SerpApiElementCarousel>),
    /// Element of type `multi_carousel`.
    #[serde(rename = "multi_carousel")]
    MultiCarousel(Box<SerpApiElementMultiCarousel>),
    /// Element of type `featured_snippet`.
    #[serde(rename = "featured_snippet")]
    FeaturedSnippet(Box<SerpApiElementFeaturedSnippet>),
    /// Element of type `knowledge_graph`.
    #[serde(rename = "knowledge_graph")]
    KnowledgeGraph(Box<SerpApiElementKnowledgeGraph>),
    /// Element of type `local_pack`.
    #[serde(rename = "local_pack")]
    LocalPack(Box<SerpApiElementLocalPack>),
    /// Element of type `hotels_pack`.
    #[serde(rename = "hotels_pack")]
    HotelsPack(Box<SerpApiElementHotelsPack>),
    /// Element of type `people_also_ask`.
    #[serde(rename = "people_also_ask")]
    PeopleAlsoAsk(Box<SerpApiElementPeopleAlsoAsk>),
    /// Element of type `organic`.
    #[serde(rename = "organic")]
    Organic(Box<SerpApiElementOrganic>),
    /// Element of type `paid`.
    #[serde(rename = "paid")]
    Paid(Box<SerpApiElementPaid>),
    /// Element of type `related_searches`.
    #[serde(rename = "related_searches")]
    RelatedSearches(Box<SerpApiElementRelatedSearches>),
    /// Element of type `people_also_search`.
    #[serde(rename = "people_also_search")]
    PeopleAlsoSearch(Box<SerpApiElementPeopleAlsoSearch>),
    /// Element of type `ai_overview`.
    #[serde(rename = "ai_overview")]
    AiOverview(Box<SerpApiElementAiOverview>),
    /// Fallback holding the raw JSON of an unrecognized `type`.
    #[serde(untagged)]
    Unknown(Value),
}

/// Google Organic Item Paid Extra SERP data model.
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

/// Hotels Pack Element Price SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementHotelsPackElementPrice {
    /// Current price.
    pub current: Option<f64>,
    /// Regular (non-discounted) price.
    pub regular: Option<f64>,
    /// Upper bound of the price range.
    pub max_value: Option<f64>,
    /// ISO currency code of the price.
    pub currency: Option<String>,
    /// `true` if the price represents a range.
    pub is_price_range: Option<bool>,
    /// Price string as displayed in the SERP.
    pub displayed_price: Option<String>,
}
/// Top Stories Element SERP data model.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementTopStoriesElement {
    /// Element type as reported by the DataForSEO API.
    #[serde(rename = "type")]
    pub type_of_element: Option<String>,
    /// Source / publisher of the result.
    pub source: Option<String>,
    /// Domain of the result.
    pub domain: Option<String>,
    /// Title of the result.
    pub title: Option<String>,
    /// Date associated with the result.
    pub date: Option<String>,
    /// `true` if an AMP version of the page is available.
    pub amp_version: Option<bool>,
    /// UTC timestamp associated with the result.
    pub timestamp: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// URL of the image.
    pub image_url: Option<String>,
    /// Badges shown on the result.
    pub badges: Option<Vec<String>>,
}
