//! Fetch Google Ads search volume for a list of keywords (task flow).
//!
//! Run with: `DATAFORSEO_LOGIN=... DATAFORSEO_PASSWORD=... cargo run --example keywords_search_volume`

use data_for_seo::{DataForSeoClient, KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DataForSeoClient::from_env()?;

    // Japan (20636), Japanese.
    let mut request =
        KeywordsDataApiGoogleAdsSearchVolumeTaskPostRequest::new("ja".to_string(), 20636);
    request.keywords = vec!["seo".to_string(), "rust".to_string()];

    let posted = client
        .keywords_data()
        .google_ads()
        .search_volume_task_post(vec![request])
        .await?;
    let task_id = posted.tasks().first().ok_or("no task posted")?.id.clone();
    println!("posted task {task_id}");

    // Google Ads tasks usually complete within seconds; poll task_get directly.
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        let response = client
            .keywords_data()
            .google_ads()
            .search_volume_task_get(&task_id)
            .await?;
        let task = response.tasks().first().ok_or("task missing")?;
        if !task.is_success() {
            println!("waiting ({})...", task.status_message);
            continue;
        }
        for result in task.results() {
            println!(
                "{}: volume={:?} cpc={:?} competition={:?}",
                result.keyword.clone().unwrap_or_default(),
                result.search_volume,
                result.cpc,
                result.competition,
            );
        }
        return Ok(());
    }
}
