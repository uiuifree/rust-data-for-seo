use serde::{Deserialize, Serialize};

/// Result of `content_generation/generate/live`: text continued from the input.
/// See <https://docs.dataforseo.com/v3/content_generation/generate/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiGenerate {
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
