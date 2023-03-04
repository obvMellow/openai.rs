use crate::client::Client;
use crate::args::{CompletionArgs, EditArgs, ImageArgs, ImageResponseFormat, ImageSize};
use crate::response::*;
use std::fs;

#[tokio::test]
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

    let resp = client.create_completion(&args).await.unwrap();

    let text = resp.get_content(0).await;

    match text {
        Some(val) => assert!(!val.is_empty()),
        None => panic!("Expected a String, got None for completion text")
    }
}

#[tokio::test]
async fn edit() {
    let client = Client::new(fs::read_to_string("key.txt")
        .unwrap()
        .as_str());

    let args = EditArgs::new(None,
        "Fix spelling mistakes",
        "What day of the wek is it?",
        Some(1),
        Some(1.0),
        Some(0.7)
    );

    let resp = client.create_edit(&args).await.unwrap();

    let text = resp.get_content(0).await;

    match text {
        Some(val) => assert!(!val.is_empty()),
        None => panic!("Expected a String, got None for edit text!")
    }
}

#[tokio::test]
async fn create_image() {
    let client = Client::new(fs::read_to_string("key.txt")
        .unwrap()
        .as_str());

    let args = ImageArgs::new("A realistic cat", Some(1), Some(ImageSize::Small), Some(ImageResponseFormat::Url));

    let resp = client.create_image(&args).await.unwrap();

    let img = resp.get_content(0).await;

    match img {
        Some(val) => assert!(val.starts_with("https://")),
        None => panic!("Expected a String, got None for image url!")
    }
}

#[test]
fn get_key() {
    let client = Client::new("key");

    assert_eq!(client.get_key(), "key");
}

#[test]
fn set_key() {
    let mut client = Client::new("key");

    client.set_key("new key");

    assert_eq!(client.get_key(), "new key");
}