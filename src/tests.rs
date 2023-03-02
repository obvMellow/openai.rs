use crate::client::{Args, Client};
use crate::response::Response;
use tokio::test;
use std::env;

#[test]
async fn completion() {
    let client = Client::new(env::args()
        .nth_back(0)
        .unwrap()
        .as_str());

    let args = Args::new("say this is a test", Option::None, Option::None, Option::None, Option::None);

    let resp = client.create_completion(args).await.unwrap().json::<Response>().await.unwrap();

    assert_eq!(resp.model, "gpt-3.5-turbo".to_string());
}