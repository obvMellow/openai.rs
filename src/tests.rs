use crate::client::Client;
use crate::args::*;
use crate::response::*;
use std::collections::HashMap;
use std::env;

#[tokio::test]
async fn completion() {
    let client = Client::new(env::var("OPENAI_API_KEY")
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
    let client = Client::new(env::var("OPENAI_API_KEY")
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
async fn image() {
    let client = Client::new(env::var("OPENAI_API_KEY")
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

#[tokio::test]
async fn models() {
    let client = Client::new(env::var("OPENAI_API_KEY")
        .unwrap()
        .as_str());

    let models = client.get_models().await.unwrap();

    assert!(models
        .as_object()
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap()
        .len() > 60);
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


#[tokio::test]
async fn chat_completion() {
    let client = Client::new(env::var("OPENAI_API_KEY")
        .unwrap()
        .as_str());

    let mut message1: HashMap<String, String> = HashMap::new();
    message1.insert("role".to_string(), "user".to_string());
    message1.insert("content".to_string(), "Who won the world series in 2020?".to_string());

    let mut message2: HashMap<String, String> = HashMap::new();
    message2.insert("role".to_string(), "system".to_string());
    message2.insert("content".to_string(), "You are a helpful assistant.".to_string());

    let messages: Vec<HashMap<String, String>> = vec![message1, message2];

    let args = ChatArgs::new(messages, None, None, None, None, None, None);

    let resp = client.create_chat_completion(args).await;

    match resp {
        Err(e) => panic!("An error occured while creating chat completion: {:?}", e),
        _ => ()
    }
}

