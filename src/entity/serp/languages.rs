use serde::{Deserialize, Serialize};

/// Language SERP data model.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SerpApiLanguage {
    /// Full language name of the search.
    pub language_name: String,
    /// Language code the search was run for.
    pub language_code: String,
}
