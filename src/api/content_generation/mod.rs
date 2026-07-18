use crate::entity::{
    ContentGenerationApiCheckGrammar, ContentGenerationApiCheckGrammarLanguage,
    ContentGenerationApiGenerate, ContentGenerationApiGenerateMetaTags,
    ContentGenerationApiGenerateSubTopics, ContentGenerationApiGenerateText,
    ContentGenerationApiGrammarRules, ContentGenerationApiParaphrase,
    ContentGenerationApiTextSummary, ContentGenerationApiTextSummaryLanguage,
};
use crate::{DataForSeoApiResponse, DataForSeoClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Content Generation API — text generation, paraphrasing, and grammar checks.
/// See <https://docs.dataforseo.com/v3/content_generation/overview/>.
pub struct ContentGenerationApi<'a> {
    client: &'a DataForSeoClient,
}

impl DataForSeoClient {
    /// Returns the Content Generation API endpoints.
    pub fn content_generation(&self) -> ContentGenerationApi<'_> {
        ContentGenerationApi { client: self }
    }
}

impl ContentGenerationApi<'_> {
    /// Continues the input text.
    /// <https://docs.dataforseo.com/v3/content_generation/generate/live/>
    pub async fn generate(
        &self,
        data: Vec<ContentGenerationApiGeneratePost>,
    ) -> DataForSeoApiResponse<ContentGenerationApiGenerate> {
        self.client
            .http_post("/v3/content_generation/generate/live", &data)
            .await
    }

    /// Generates text from a topic and parameters.
    /// <https://docs.dataforseo.com/v3/content_generation/generate_text/live/>
    pub async fn generate_text(
        &self,
        data: Vec<ContentGenerationApiGenerateTextPost>,
    ) -> DataForSeoApiResponse<ContentGenerationApiGenerateText> {
        self.client
            .http_post("/v3/content_generation/generate_text/live", &data)
            .await
    }

    /// Generates title and description meta tags for the input text.
    /// <https://docs.dataforseo.com/v3/content_generation/generate_meta_tags/live/>
    pub async fn generate_meta_tags(
        &self,
        data: Vec<ContentGenerationApiGenerateMetaTagsPost>,
    ) -> DataForSeoApiResponse<ContentGenerationApiGenerateMetaTags> {
        self.client
            .http_post("/v3/content_generation/generate_meta_tags/live", &data)
            .await
    }

    /// Suggests subtopics for a topic.
    /// <https://docs.dataforseo.com/v3/content_generation/generate_sub_topics/live/>
    pub async fn generate_sub_topics(
        &self,
        data: Vec<ContentGenerationApiGenerateSubTopicsPost>,
    ) -> DataForSeoApiResponse<ContentGenerationApiGenerateSubTopics> {
        self.client
            .http_post("/v3/content_generation/generate_sub_topics/live", &data)
            .await
    }

    /// Rewrites the input text.
    /// <https://docs.dataforseo.com/v3/content_generation/paraphrase/live/>
    pub async fn paraphrase(
        &self,
        data: Vec<ContentGenerationApiParaphrasePost>,
    ) -> DataForSeoApiResponse<ContentGenerationApiParaphrase> {
        self.client
            .http_post("/v3/content_generation/paraphrase/live", &data)
            .await
    }

    /// Checks the grammar and spelling of the input text.
    /// <https://docs.dataforseo.com/v3/content_generation/check_grammar/live/>
    pub async fn check_grammar(
        &self,
        data: Vec<ContentGenerationApiCheckGrammarPost>,
    ) -> DataForSeoApiResponse<ContentGenerationApiCheckGrammar> {
        self.client
            .http_post("/v3/content_generation/check_grammar/live", &data)
            .await
    }

    /// Languages supported by the Check Grammar endpoint.
    /// <https://docs.dataforseo.com/v3/content_generation/check_grammar/languages/>
    pub async fn check_grammar_languages(
        &self,
    ) -> DataForSeoApiResponse<ContentGenerationApiCheckGrammarLanguage> {
        self.client
            .http_get("/v3/content_generation/check_grammar/languages")
            .await
    }

    /// Grammar and spelling rules used by the Check Grammar endpoint.
    /// <https://docs.dataforseo.com/v3/content_generation/grammar_rules/>
    pub async fn grammar_rules(&self) -> DataForSeoApiResponse<ContentGenerationApiGrammarRules> {
        self.client
            .http_post("/v3/content_generation/grammar_rules", &Vec::<Value>::new())
            .await
    }

    /// Analyzes structure and readability of the input text.
    /// <https://docs.dataforseo.com/v3/content_generation/text_summary/live/>
    pub async fn text_summary(
        &self,
        data: Vec<ContentGenerationApiTextSummaryPost>,
    ) -> DataForSeoApiResponse<ContentGenerationApiTextSummary> {
        self.client
            .http_post("/v3/content_generation/text_summary/live", &data)
            .await
    }

    /// Languages supported by the Text Summary endpoint.
    /// <https://docs.dataforseo.com/v3/content_generation/text_summary/languages/>
    pub async fn text_summary_languages(
        &self,
    ) -> DataForSeoApiResponse<ContentGenerationApiTextSummaryLanguage> {
        self.client
            .http_get("/v3/content_generation/text_summary/languages")
            .await
    }
}

/// Request for `content_generation/generate/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiGeneratePost {
    /// Initial text to continue (1–500 tokens).
    pub text: String,
    /// Maximum new tokens to generate (max 300); required unless `max_tokens`
    /// is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_new_tokens: Option<u32>,
    /// Maximum total tokens including the input (max 1024); required unless
    /// `max_new_tokens` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Randomness of token selection, 0–1 (default 0.8); overrides
    /// top_k/top_p/temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creativity_index: Option<f64>,
    /// Penalty limiting repeated tokens, 0.5–2 (default 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_repetition_penalty: Option<f64>,
    /// Number of top tokens shortlisted per step, 1–100 (default 40); ignored
    /// when `creativity_index` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    /// Nucleus sampling probability cutoff, 0–1 (default 0.9); ignored when
    /// `creativity_index` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Output randomness, 0–1 (default 0.7); ignored when `creativity_index` is
    /// set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Up to 50 words or phrases to avoid anywhere in the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avoid_words: Option<Vec<String>>,
    /// Up to 50 words or phrases to avoid at the start of the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avoid_starting_words: Option<Vec<String>>,
    /// Up to 50 words or phrases that end generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_words: Option<Vec<String>>,
    /// Token from a previous response to continue that generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplement_token: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentGenerationApiGeneratePost {
    /// Creates a request that continues the given text.
    pub fn new(text: &str) -> ContentGenerationApiGeneratePost {
        ContentGenerationApiGeneratePost {
            text: text.to_string(),
            ..Default::default()
        }
    }
}

/// Request for `content_generation/generate_text/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiGenerateTextPost {
    /// Main topic of the content (1–50 tokens).
    pub topic: String,
    /// Target number of words, 1–1000.
    pub word_count: u32,
    /// Up to 10 secondary topics to cover.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_topics: Option<Vec<String>>,
    /// Meta description guiding the content (1–1000 tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Up to 10 keywords to weave into the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_keywords: Option<Vec<String>>,
    /// Randomness of token selection, 0–1 (default 0.8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creativity_index: Option<f64>,
    /// Append a logical conclusion when true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_conclusion: Option<bool>,
    /// Token from a previous response to continue that generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplement_token: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentGenerationApiGenerateTextPost {
    /// Creates a request for the given topic and target word count.
    pub fn new(topic: &str, word_count: u32) -> ContentGenerationApiGenerateTextPost {
        ContentGenerationApiGenerateTextPost {
            topic: topic.to_string(),
            word_count,
            ..Default::default()
        }
    }
}

/// Request for `content_generation/generate_meta_tags/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiGenerateMetaTagsPost {
    /// Input text to generate meta tags for (1–500 tokens).
    pub text: String,
    /// Randomness of token selection, 0–1 (default 0.8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creativity_index: Option<f64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentGenerationApiGenerateMetaTagsPost {
    /// Creates a request that generates meta tags for the given text.
    pub fn new(text: &str) -> ContentGenerationApiGenerateMetaTagsPost {
        ContentGenerationApiGenerateMetaTagsPost {
            text: text.to_string(),
            ..Default::default()
        }
    }
}

/// Request for `content_generation/generate_sub_topics/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiGenerateSubTopicsPost {
    /// Main topic to generate subtopics for (1–50 tokens).
    pub topic: String,
    /// Randomness of token selection, 0–1 (default 0.8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creativity_index: Option<f64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentGenerationApiGenerateSubTopicsPost {
    /// Creates a request that suggests subtopics for the given topic.
    pub fn new(topic: &str) -> ContentGenerationApiGenerateSubTopicsPost {
        ContentGenerationApiGenerateSubTopicsPost {
            topic: topic.to_string(),
            ..Default::default()
        }
    }
}

/// Request for `content_generation/paraphrase/live`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiParaphrasePost {
    /// Text to paraphrase (1–500 tokens).
    pub text: String,
    /// Randomness of token selection, 0–1 (default 0.8).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creativity_index: Option<f64>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentGenerationApiParaphrasePost {
    /// Creates a request that paraphrases the given text.
    pub fn new(text: &str) -> ContentGenerationApiParaphrasePost {
        ContentGenerationApiParaphrasePost {
            text: text.to_string(),
            ..Default::default()
        }
    }
}

/// Request for `content_generation/check_grammar/live`.
///
/// Set exactly one of `language_code` or `language_name`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiCheckGrammarPost {
    /// Text to check (1–10,000 tokens).
    pub text: String,
    /// Language code of the text; required unless `language_name` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Language name of the text; required unless `language_code` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentGenerationApiCheckGrammarPost {
    /// Creates a request that checks the grammar of the given text.
    pub fn new(text: &str) -> ContentGenerationApiCheckGrammarPost {
        ContentGenerationApiCheckGrammarPost {
            text: text.to_string(),
            ..Default::default()
        }
    }
}

/// Request for `content_generation/text_summary/live`.
///
/// Set exactly one of `language_name` or `language_code`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ContentGenerationApiTextSummaryPost {
    /// Text to analyze (1–10,000 tokens).
    pub text: String,
    /// Language name of the text; required unless `language_code` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// Language code of the text; required unless `language_name` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Maximum elements in the keyword_density array (default 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_list_limit: Option<u32>,
    /// User-defined task identifier (max 255 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ContentGenerationApiTextSummaryPost {
    /// Creates a request that summarizes the given text.
    pub fn new(text: &str) -> ContentGenerationApiTextSummaryPost {
        ContentGenerationApiTextSummaryPost {
            text: text.to_string(),
            ..Default::default()
        }
    }
}
