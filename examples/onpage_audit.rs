//! Crawl a website with the OnPage API and report technical SEO issues.
//!
//! Flow: task_post → poll summary until the crawl finishes → fetch pages.
//! Run with: `DATAFORSEO_LOGIN=... DATAFORSEO_PASSWORD=... cargo run --example onpage_audit`

use data_for_seo::{DataForSeoClient, OnPageApiPagesPost, OnPageApiTaskPost};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DataForSeoClient::from_env()?;

    // 1. Start a crawl (10 pages max for this demo).
    let mut request = OnPageApiTaskPost::new("example.com".to_string(), 10);
    request.enable_javascript = Some(true);
    let posted = client.on_page().task_post(vec![request]).await?;
    let task_id = posted.tasks().first().ok_or("no task posted")?.id.clone();
    println!("crawl started: {task_id}");

    // 2. Poll the summary until the crawl is finished.
    let summary = loop {
        tokio::time::sleep(Duration::from_secs(15)).await;
        let response = client.on_page().summary(&task_id).await?;
        if let Some(summary) = response.first_result() {
            let progress = summary.crawl_progress.clone().unwrap_or_default();
            println!("crawl progress: {progress}");
            if progress == "finished" {
                break summary.clone();
            }
        }
    };
    if let Some(info) = &summary.domain_info {
        println!("crawled {:?} pages", info.total_pages);
    }

    // 3. Fetch crawled pages with their OnPage scores.
    let mut request = OnPageApiPagesPost::new(task_id);
    request.limit = Some(10);
    let pages = client.on_page().pages(vec![request]).await?;
    for result in pages.results() {
        for page in result.items.iter().flatten() {
            println!(
                "{:?} status={:?} score={:?}",
                page.url, page.status_code, page.onpage_score,
            );
        }
    }
    Ok(())
}
