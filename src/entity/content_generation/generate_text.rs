use serde::{Deserialize, Serialize};

/// Result of `content_generation/generate_text/live`: text generated from a topic.
/// See <https://docs.dataforseo.com/v3/content_generation/generate_text/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiGenerateText {
    /// Number of input tokens.
    pub input_tokens: Option<i64>,
    /// Number of output tokens.
    pub output_tokens: Option<i64>,
    /// Number of newly generated tokens.
    pub new_tokens: Option<i64>,
    /// Resulting text.
    pub generated_text: Option<String>,
    /// Token to continue this generation from its end in a follow-up request.
    pub supplement_token: Option<String>,
}
