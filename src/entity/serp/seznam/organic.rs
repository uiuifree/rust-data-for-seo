use crate::entity::{SerpApiGoogleOrganicItem, SerpApiHtmlItem, SerpApiSearchResult};

/// Seznam organic advanced result. Items reuse the shared organic item enum;
/// unrecognized item types fall back to [`SerpApiGoogleOrganicItem::Unknown`].
/// See <https://docs.dataforseo.com/v3/serp/seznam/organic/task_get/advanced/>.
pub type SerpApiSeznamOrganicTaskAdvanced = SerpApiSearchResult<SerpApiGoogleOrganicItem>;
/// Seznam organic raw-HTML result.
/// See <https://docs.dataforseo.com/v3/serp/seznam/organic/task_get/html/>.
pub type SerpApiSeznamOrganicTaskHtml = SerpApiSearchResult<SerpApiHtmlItem>;
