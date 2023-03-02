use crate::client::{Args, Client};
use serde_json::Value;
use tokio::test;
use std::fs;

#[test]
async fn completion() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new(fs::read_to_string("key.txt")
        .unwrap()
        .as_str());

    let args = Args::new("say this is a test", Option::None, Option::None, Option::None, Option::None);

    let resp = client.create_completion(args)
        .await?
        .json::<Value>()
        .await?;

    let resp = resp.as_object().unwrap();

    assert_eq!(resp
        .get("model")
        .unwrap()
        .as_str()
        .unwrap(), "text-davinci-003");

    let choices = resp["choices"].as_array().unwrap();

    assert_eq!(choices
        .get(0)
        .unwrap()
        .as_object()
        .unwrap()
        .get("index")
        .unwrap()
        .as_i64()
        .unwrap(), 0);

    Ok(())
}