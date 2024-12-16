use data_for_seo::DataForSeoClient;
use std::env;

fn client() -> DataForSeoClient {
    dotenv::dotenv().ok();
    let id = env::var("ID").unwrap();
    let pass = env::var("PASSWORD").unwrap();

    DataForSeoClient::new(id, pass)
}

#[tokio::test]
async fn test_task_ready() {
    let client = client();
    let tasks = client.serp().task_ready().await.unwrap().tasks;
    assert_ne!(tasks.len(), 0);
    for task in tasks {
        println!("{:}", task.id);
        let results = task.result.unwrap();
        assert_ne!(results.len(), 0);
        for result in results {
            println!("{:?}", result);
        }
    }
}
#[tokio::test]
async fn test_task_ready_google() {
    let client = client();
    let tasks = client.serp().task_ready_se("google").await.unwrap().tasks;
    assert_ne!(tasks.len(), 0);
    for task in tasks {
        println!("{:}", task.id);
        let results = task.result.unwrap();
        assert_ne!(results.len(), 0);
        for result in results {
            println!("{:?}", result);
        }
    }
}
