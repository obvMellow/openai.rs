use crate::client::{Args, Client};
use crate::response::Response;
use tokio::test;

#[test]
async fn completion() {
    let client = Client::new("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");

    let args = Args::new("say this is a test", Option::None, Option::None, Option::None, Option::None);

    let resp = client.create_completion(args).await.unwrap().json::<Response>().await.unwrap();

    assert_eq!(resp.model, "gpt-3.5-turbo".to_string());
}