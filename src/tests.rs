use crate::client::Client;
use crate::args::{CompletionArgs, EditArgs, ImageArgs, ImageResponseFormat, ImageSize};
use serde_json::Value;
use tokio::test;
use std::fs;

#[test]
async fn completion() {
    let client = Client::new(fs::read_to_string("key.txt")
        .unwrap()
        .as_str());

    let args = CompletionArgs::new("Say this is a test",
        Some(32),
        Some(2),
        None,
        Some(1.0)
    );

    let resp = client.create_completion(&args)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    let resp = resp.as_object().unwrap();

    assert_eq!(resp
        .get("object")
        .unwrap()
        .as_str()
        .unwrap(), "text_completion");
}

#[test]
async fn edit() {
    let client = Client::new(fs::read_to_string("key.txt")
        .unwrap()
        .as_str());

    let args = EditArgs::new(None,
        "Fix spelling mistakes".to_string(),
        Some("What day of the wek is it?".to_string()),
        Some(1),
        Some(1.0),
        Some(0.7)
    );

    let resp = client.create_edit(&args)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    assert_eq!(resp.
        get("object")
        .unwrap()
        .as_str()
        .unwrap(), "edit");
}

#[test]
async fn create_image() {
    let client = Client::new(fs::read_to_string("key.txt")
        .unwrap()
        .as_str());

    let args = ImageArgs::new("A realistic cat".to_string(), Some(1), Some(ImageSize::Small), Some(ImageResponseFormat::Url));

    let resp = client.create_image(&args)
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    let url = resp
        .get("data")
        .unwrap()
        .as_array()
        .unwrap()
        .get(0)
        .unwrap()
        .as_object()
        .unwrap()
        .get("url")
        .unwrap()
        .as_str()
        .unwrap();

    assert!(url.starts_with("https://"));
}
