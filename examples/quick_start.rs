//! Minimal example: fetch a backlinks summary and a Google organic SERP.
//!
//! Run with: `ID=... PASSWORD=... cargo run --example quick_start`

use data_for_seo::{
    BacklinksApiSummaryPost, DataForSeoClient, SerpApiGoogleOrganicTaskPostRequest,
};

#[tokio::main]
async fn main() -> Result<(), data_for_seo::error::DataForSeoError> {
    let id = std::env::var("ID").unwrap_or_default();
    let password = std::env::var("PASSWORD").unwrap_or_default();
    let client = DataForSeoClient::new(id, password);

    // Live endpoint: backlinks summary
    let response = client
        .backlinks()
        .summary(vec![BacklinksApiSummaryPost::new("https://example.com/")])
        .await?;
    for result in response.results() {
        println!("backlinks: {:?}", result.backlinks);
    }

    // Task-based endpoint: post a Google organic task
    let mut request = SerpApiGoogleOrganicTaskPostRequest::new("ja".to_string(), 2392);
    request.keyword = "seo".to_string();
    let posted = client
        .serp()
        .google()
        .organic_task_post(vec![request])
        .await?;
    if let Some(task) = posted.tasks().first() {
        println!("task id: {} status: {}", task.id, task.status_message);

        // ... after polling tasks_ready:
        let result = client
            .serp()
            .google()
            .organic_task_get_advanced(&task.id)
            .await?;
        for page in result.results() {
            for item in page.items() {
                println!("{:?}", item);
            }
        }
    }
    Ok(())
}
