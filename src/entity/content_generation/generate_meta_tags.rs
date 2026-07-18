use serde::{Deserialize, Serialize};

/// Result of `content_generation/generate_meta_tags/live`: title and description
/// meta tags generated for the input text.
/// See <https://docs.dataforseo.com/v3/content_generation/generate_meta_tags/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiGenerateMetaTags {
    /// Number of input tokens.
    pub input_tokens: Option<i64>,
    /// Number of output tokens.
    pub output_tokens: Option<i64>,
    /// Number of newly generated tokens.
    pub new_tokens: Option<i64>,
    /// Generated title meta tag.
    pub title: Option<String>,
    /// Generated description meta tag.
    pub description: Option<String>,
}
