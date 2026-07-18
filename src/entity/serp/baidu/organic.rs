use crate::entity::{SerpApiGoogleOrganicItem, SerpApiHtmlItem, SerpApiSearchResult};

/// Baidu organic advanced result. Items reuse the shared organic item enum;
/// Baidu-specific item types fall back to [`SerpApiGoogleOrganicItem::Unknown`].
/// See <https://docs.dataforseo.com/v3/serp/baidu/organic/task_get/advanced/>.
pub type SerpApiBaiduOrganicTaskAdvanced = SerpApiSearchResult<SerpApiGoogleOrganicItem>;
/// Baidu organic raw-HTML result.
/// See <https://docs.dataforseo.com/v3/serp/baidu/organic/task_get/html/>.
pub type SerpApiBaiduOrganicTaskHtml = SerpApiSearchResult<SerpApiHtmlItem>;
