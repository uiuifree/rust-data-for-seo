use crate::entity::{SerpApiGoogleOrganicItem, SerpApiHtmlItem, SerpApiSearchResult};

/// Google Search By Image advanced result. Items (`organic`, `images`, ...) reuse
/// the shared organic item enum; unmodeled types fall back to
/// [`SerpApiGoogleOrganicItem::Unknown`].
/// See <https://docs.dataforseo.com/v3/serp/google/search_by_image/task_get/advanced/>.
pub type SerpApiGoogleSearchByImageTaskAdvanced = SerpApiSearchResult<SerpApiGoogleOrganicItem>;
/// Google Search By Image raw-HTML result.
/// See <https://docs.dataforseo.com/v3/serp/google/search_by_image/task_get/html/>.
pub type SerpApiGoogleSearchByImageTaskHtml = SerpApiSearchResult<SerpApiHtmlItem>;
