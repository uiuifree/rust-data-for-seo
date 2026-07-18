use serde::{Deserialize, Serialize};

/// Result item of `content_analysis/languages`: a language the API can filter by.
/// See <https://docs.dataforseo.com/v3/content_analysis/languages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentAnalysisApiLanguage {
    /// Full language name.
    pub language_name: Option<String>,
    /// ISO 639-1 language code.
    pub language_code: Option<String>,
}
