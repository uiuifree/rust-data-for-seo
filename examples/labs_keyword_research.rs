//! Keyword research with DataForSEO Labs: ideas and related keywords with
//! search volume, competition, and difficulty.
//!
//! Run with: `DATAFORSEO_LOGIN=... DATAFORSEO_PASSWORD=... cargo run --example labs_keyword_research`

use data_for_seo::{
    DataForSeoClient, DataForSeoLabsKeywordIdeasRequest, DataForSeoLabsRelatedKeywordsRequest,
};

#[tokio::main]
async fn main() -> Result<(), data_for_seo::error::DataForSeoError> {
    let client = DataForSeoClient::from_env()?;

    // Keyword ideas: keywords from the same category as the seeds.
    let mut request = DataForSeoLabsKeywordIdeasRequest::new(vec!["seo tools".to_string()]);
    request.location_code = Some(2840); // United States
    request.language_code = Some("en".to_string());
    request.limit = Some(10);

    let ideas = client
        .dataforseo_labs()
        .google()
        .keyword_ideas(vec![request])
        .await?;
    for result in ideas.results() {
        for item in result.items.iter().flatten() {
            let info = item.keyword_info.as_ref();
            println!(
                "idea: {:<30} volume={:?} cpc={:?} competition={:?}",
                item.keyword.clone().unwrap_or_default(),
                info.and_then(|i| i.search_volume),
                info.and_then(|i| i.cpc),
                info.and_then(|i| i.competition),
            );
        }
    }

    // Related keywords: mined from "searches related to" (depth 0-4).
    let mut request = DataForSeoLabsRelatedKeywordsRequest::new("seo tools");
    request.location_code = Some(2840);
    request.language_code = Some("en".to_string());
    request.limit = Some(10);

    let related = client
        .dataforseo_labs()
        .google()
        .related_keywords(vec![request])
        .await?;
    for result in related.results() {
        for item in result.items.iter().flatten() {
            if let Some(data) = &item.keyword_data {
                println!(
                    "related: {:<30} volume={:?}",
                    data.keyword.clone().unwrap_or_default(),
                    data.keyword_info.as_ref().and_then(|i| i.search_volume),
                );
            }
        }
    }
    Ok(())
}
