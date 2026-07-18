//! Audit a domain's backlink profile: summary, top referring domains, newest links.
//!
//! All Backlinks endpoints are live (no task flow needed).
//! Run with: `DATAFORSEO_LOGIN=... DATAFORSEO_PASSWORD=... cargo run --example backlinks_overview`

use data_for_seo::{
    BacklinksApiBacklinksPost, BacklinksApiReferringDomainsPost, BacklinksApiSummaryPost,
    DataForSeoClient,
};

#[tokio::main]
async fn main() -> Result<(), data_for_seo::error::DataForSeoError> {
    let client = DataForSeoClient::from_env()?;
    let target = "example.com";

    // Profile summary.
    let summary = client
        .backlinks()
        .summary(vec![BacklinksApiSummaryPost::new(target)])
        .await?;
    if let Some(s) = summary.first_result() {
        println!(
            "{target}: rank={:?} backlinks={:?} referring_domains={:?} broken={:?}",
            s.rank, s.backlinks, s.referring_domains, s.broken_backlinks,
        );
    }

    // Top referring domains by rank.
    let mut request = BacklinksApiReferringDomainsPost::new(target);
    request.limit = Some(10);
    request.order_by = Some(vec!["rank,desc".to_string()]);
    let domains = client.backlinks().referring_domains(vec![request]).await?;
    for result in domains.results() {
        for item in result.items.iter().flatten() {
            println!("ref domain: {:?} (rank {:?})", item.domain, item.rank);
        }
    }

    // Newest backlinks.
    let mut request = BacklinksApiBacklinksPost::new(target);
    request.limit = Some(10);
    request.order_by = Some(vec!["first_seen,desc".to_string()]);
    let links = client.backlinks().backlinks(vec![request]).await?;
    for result in links.results() {
        for item in result.items.iter().flatten() {
            println!(
                "new link: {} -> {} (dofollow: {:?})",
                item.url_from.clone().unwrap_or_default(),
                item.url_to.clone().unwrap_or_default(),
                item.dofollow,
            );
        }
    }
    Ok(())
}
