use crate::entity::{SerpApiGoogleOrganicItem, SerpApiSearchResult};

/// Google AI Mode advanced result. AI Mode is a live-only endpoint; its items
/// reuse the shared organic item enum (`ai_overview` and others).
/// See <https://docs.dataforseo.com/v3/serp/google/ai_mode/live/advanced/>.
pub type SerpApiGoogleAiModeTaskAdvanced = SerpApiSearchResult<SerpApiGoogleOrganicItem>;
