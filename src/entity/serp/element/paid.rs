use crate::entity::serp::element::images_element::SerpApiElementImagesElement;
use crate::entity::{
    SerpApiElementLinkElement, SerpApiElementRelatedResult, SerpApiGoogleOrganicItemPaidExtra,
    SerpApiPrice,
};
use serde::{Deserialize, Serialize};

/// Paid
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SerpApiElementPaid {
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
    /// Domain of the result.
    pub domain: Option<String>,
    /// URL of the result.
    pub url: Option<String>,
    /// Breadcrumb shown under the result URL.
    pub breadcrumb: Option<String>,
    /// `true` if the result contains an image.
    pub is_image: Option<bool>,
    /// `true` if the result contains a video.
    pub is_video: Option<bool>,
    /// Images attached to the result.
    pub images: Option<Vec<SerpApiElementImagesElement>>,
    /// Substrings the search engine highlighted in the result.
    pub highlighted: Option<Vec<String>>,
    /// Extra.
    pub extra: Option<SerpApiGoogleOrganicItemPaidExtra>,
    /// Snippet / description text of the result.
    pub description: Option<String>,
    /// Description rows.
    pub description_rows: Option<String>,
    /// Links associated with the result.
    pub links: Option<Vec<SerpApiElementLinkElement>>,
    /// Price information shown for the result.
    pub price: Option<SerpApiPrice>,
    /// Related result shown alongside this one.
    pub related_result: Option<SerpApiElementRelatedResult>,
}
