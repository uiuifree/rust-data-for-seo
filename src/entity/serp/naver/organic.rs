use crate::entity::{SerpApiGoogleOrganicItem, SerpApiHtmlItem, SerpApiSearchResult};

/// Naver organic advanced result. Items reuse the shared organic item enum;
/// Naver-specific item types fall back to [`SerpApiGoogleOrganicItem::Unknown`].
/// See <https://docs.dataforseo.com/v3/serp/naver/organic/task_get/advanced/>.
pub type SerpApiNaverOrganicTaskAdvanced = SerpApiSearchResult<SerpApiGoogleOrganicItem>;
/// Naver organic raw-HTML result.
/// See <https://docs.dataforseo.com/v3/serp/naver/organic/task_get/html/>.
pub type SerpApiNaverOrganicTaskHtml = SerpApiSearchResult<SerpApiHtmlItem>;
