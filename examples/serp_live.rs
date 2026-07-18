//! Check Google rankings for a keyword with a single live request.
//!
//! Run with: `DATAFORSEO_LOGIN=... DATAFORSEO_PASSWORD=... cargo run --example serp_live`

use data_for_seo::{
    DataForSeoClient, SerpApiGoogleOrganicItem, SerpApiGoogleOrganicTaskPostRequest,
};

#[tokio::main]
async fn main() -> Result<(), data_for_seo::error::DataForSeoError> {
    let client = DataForSeoClient::from_env()?;

    // Tokyo (location_code 1009309), Japanese. Find codes via
    // client.serp().google().locations() / .languages().
    let mut request = SerpApiGoogleOrganicTaskPostRequest::new("ja".to_string(), 1009309);
    request.keyword = "seo".to_string();
    request.depth = Some(100);

    let response = client
        .serp()
        .google()
        .organic_live_advanced(vec![request])
        .await?;

    for page in response.results() {
        println!(
            "SERP for '{}' ({} results)",
            page.keyword,
            page.items_count.unwrap_or(0)
        );
        for item in page.items() {
            match item {
                SerpApiGoogleOrganicItem::Organic(organic) => {
                    println!(
                        "#{:<3} {} — {}",
                        organic.rank_absolute.unwrap_or(0),
                        organic.title.clone().unwrap_or_default(),
                        organic.url.clone().unwrap_or_default(),
                    );
                }
                SerpApiGoogleOrganicItem::PeopleAlsoAsk(paa) => {
                    println!(
                        "PAA: {} questions",
                        paa.items.as_deref().unwrap_or_default().len()
                    );
                }
                // Unknown keeps raw JSON for item types this crate does not model yet.
                SerpApiGoogleOrganicItem::Unknown(value) => {
                    println!("other: {}", value["type"].as_str().unwrap_or("?"));
                }
                _ => {}
            }
        }
    }
    Ok(())
}
