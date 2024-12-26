use data_for_seo::DataForSeoClient;
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();

    DataForSeoClient::new(id, pass)
}

#[cfg(test)]
mod on_page {
    use crate::client;
    use data_for_seo::{OnPageApiPagesByResourcePost, OnPageApiPagesPost, OnPageApiRawHtmlPost, OnPageApiTaskPost, OnPageApiWaterfallPost};

    #[tokio::test]
    async fn post() {
        let client = client();
        let mut request = OnPageApiTaskPost::new("example.com".to_string(), 1);
        request.start_url = Some("https://example.com/".to_string());
        request.validate_micromarkup = Some(true);
        request.store_raw_html = Some(true);
        let res = client.on_page().task_post(vec![request]).await;

    }
    #[tokio::test]
    async fn page_by_resource() {
        let client = client();
        let id = "xxx".to_string();
        let url = "https://example.com".to_string();

        let res = OnPageApiPagesByResourcePost::new(id, url);
        let res = client.on_page().pages_by_resource(vec![res]).await.unwrap();
        for task in res.tasks {
            println!("{:?}", task);
            for result in task.result.unwrap() {
                println!("{:?}", result);
            }
        }
    }
  #[tokio::test]
    async fn pages() {
        let client = client();
        let id = "xxx".to_string();

        let res = OnPageApiPagesPost::new(id);
        let res = client.on_page().pages(vec![res]).await.unwrap();
        for task in res.tasks {
            for result in task.result.unwrap() {
                println!("{:?}", result);
            }
        }
    }
    #[tokio::test]
    async fn waterfall() {
        let client = client();
        let id = "xxx".to_string();
        let url = "https://example.com".to_string();

        let res = OnPageApiWaterfallPost::new(id, url);
        let res = client.on_page().waterfall(vec![res]).await.unwrap();
        for task in res.tasks {
            for result in task.result.unwrap() {
                println!("{:?}", result);
            }
        }
    }    #[tokio::test]
    async fn raw_html() {
        let client = client();
        let id = "xxx".to_string();
        let url = "https://example.com".to_string();

        let res = OnPageApiRawHtmlPost::new(id, url);
        let res = client.on_page().raw_html(vec![res]).await.unwrap();
        for task in res.tasks {
            for result in task.result.unwrap() {
                println!("{:?}", result);
            }
        }
    }
}
