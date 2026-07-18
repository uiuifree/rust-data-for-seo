//! The complete task flow: post a task, poll until it is ready, fetch the result.
//!
//! Cheaper than live requests; results are stored for 30 days.
//! Run with: `DATAFORSEO_LOGIN=... DATAFORSEO_PASSWORD=... cargo run --example serp_task_flow`

use data_for_seo::entity::TaskStatus;
use data_for_seo::{DataForSeoClient, SerpApiGoogleOrganicTaskPostRequest};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DataForSeoClient::from_env()?;

    // 1. Post the task.
    let mut request = SerpApiGoogleOrganicTaskPostRequest::new("ja".to_string(), 2392);
    request.keyword = "rust seo".to_string();
    let posted = client
        .serp()
        .google()
        .organic_task_post(vec![request])
        .await?;
    let task_id = posted.tasks().first().ok_or("no task posted")?.id.clone();
    println!("posted task {task_id}");

    // 2. Poll `tasks_ready` until our task shows up (typically well under a minute).
    'wait: loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
        let ready = client.serp().task_ready().await?;
        for task in ready.tasks() {
            for result in task.results() {
                if result.id.as_deref() == Some(task_id.as_str()) {
                    break 'wait;
                }
            }
        }
        println!("not ready yet...");
    }

    // 3. Fetch the stored result. `task_status()` classifies DataForSEO status codes,
    //    so this also works when polling `task_get` directly instead of `tasks_ready`.
    let result = client
        .serp()
        .google()
        .organic_task_get_advanced(&task_id)
        .await?;
    for task in result.tasks() {
        if let TaskStatus::RequestSuccess(_) = task.task_status() {
            for page in task.results() {
                println!("'{}': {} items", page.keyword, page.items().len());
            }
        }
    }
    Ok(())
}
