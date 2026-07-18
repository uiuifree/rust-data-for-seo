use crate::entity::{SerpApiGoogleOrganicItem, SerpApiHtmlItem, SerpApiSearchResult};

/// Yahoo organic advanced result. Items reuse the shared organic item enum;
/// unrecognized item types fall back to [`SerpApiGoogleOrganicItem::Unknown`].
/// See <https://docs.dataforseo.com/v3/serp/yahoo/organic/task_get/advanced/>.
pub type SerpApiYahooOrganicTaskAdvanced = SerpApiSearchResult<SerpApiGoogleOrganicItem>;
/// Yahoo organic raw-HTML result.
/// See <https://docs.dataforseo.com/v3/serp/yahoo/organic/task_get/html/>.
pub type SerpApiYahooOrganicTaskHtml = SerpApiSearchResult<SerpApiHtmlItem>;
