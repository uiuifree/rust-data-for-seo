use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of `content_generation/text_summary/live`: structural and readability
/// statistics for the target text.
/// See <https://docs.dataforseo.com/v3/content_generation/text_summary/live/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiTextSummary {
    /// Number of sentences in the text.
    pub sentences: Option<i64>,
    /// Number of paragraphs in the text.
    pub paragraphs: Option<i64>,
    /// Number of words in the text.
    pub words: Option<i64>,
    /// Number of characters excluding spaces.
    pub characters_without_spaces: Option<i64>,
    /// Number of characters including spaces.
    pub characters_with_spaces: Option<i64>,
    /// Average number of words per sentence.
    pub words_per_sentence: Option<f64>,
    /// Average number of characters per word.
    pub characters_per_word: Option<f64>,
    /// Vocabulary density of the text.
    pub vocabulary_density: Option<f64>,
    /// Most common words mapped to their occurrence count.
    pub keyword_density: Option<HashMap<String, i64>>,
    /// Automated Readability Index.
    pub automated_readability_index: Option<f64>,
    /// Coleman–Liau Index.
    pub coleman_liau_index: Option<f64>,
    /// Flesch–Kincaid grade level.
    pub flesch_kincaid_grade_level: Option<f64>,
    /// SMOG readability index.
    pub smog_readability_index: Option<f64>,
    /// Number of spelling errors found.
    pub spelling_errors: Option<i64>,
    /// Number of grammar errors found.
    pub grammar_errors: Option<i64>,
}

/// Result item of `content_generation/text_summary/languages`: a supported
/// text-summary language.
/// See <https://docs.dataforseo.com/v3/content_generation/text_summary/languages/>.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiTextSummaryLanguage {
    /// Full language name.
    pub language_name: Option<String>,
    /// ISO 639-1 language code.
    pub language_code: Option<String>,
}
