use data_for_seo::DataForSeoClient;
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();

    DataForSeoClient::new(id, pass)
}

#[cfg(test)]
mod backlinks {
    use crate::client;
    use data_for_seo::{BacklinksApiAnchorPost, BacklinksApiBacklinksPost, BacklinksApiSummaryPost};

    #[tokio::test]
    async fn index() {
        let client = client();
        let res = client.backlinks().index().await.unwrap();
        for task in res.tasks {
            for res in task.result.unwrap() {
                println!("total_backlinks: {:?}", res.total_backlinks);
                println!("total_pages: {:?}", res.total_pages);
                println!("total_domains: {:?}", res.total_domains);
            }
        }
    }
    #[tokio::test]
    async fn summary() {
        let client = client();
        let request = BacklinksApiSummaryPost::new("https://example.com/");
        let res = client.backlinks().summary(vec![request]).await.unwrap();
        for task in res.tasks {
            for res in task.result.unwrap() {
                println!("res: {:?}", res);
            }
        }
    }
    #[tokio::test]
    async fn backlinks() {
        let client = client();
        let mut request = BacklinksApiBacklinksPost::new("https://example.com/");
        // request.mode = Some("one_per_domain".to_string());
        request.order_by = Some(vec!["rank,desc".to_string()]);
        request.limit = Some(100);
        let res = client.backlinks().backlinks(vec![request]).await.unwrap();
        for task in res.tasks {
            for res in task.result.unwrap() {
                println!("mode: {:?}", res.mode);
                println!("items_count: {:?}", res.items_count);
                for item in res.items.unwrap(){
                    println!("res: {:?}", item);

                }
            }
        }
    }
    #[tokio::test]
    async fn anchor() {
        let client = client();
        let request = BacklinksApiAnchorPost::new("https://example.com/");
        let res = client.backlinks().anchor(vec![request]).await.unwrap();
        for task in res.tasks {
            for res in task.result.unwrap() {
                for item in res.items.unwrap(){
                    println!("res: {:?}", item);

                }
            }
        }
    }
}
