use crate::args::*;
use crate::client::Client;
use crate::response::*;
use std::collections::HashMap;
use std::env;

#[tokio::test]
async fn completion() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let resp = client
        .create_completion(|args| {
            args.prompt("Once upon a time")
                .model("text-davinci-003")
                .max_tokens(32)
                .n(1)
                .temperature(1.0)
        })
        .await
        .unwrap();

    let text = resp.get_content(0).await;

    match text {
        Some(val) => assert!(!val.is_empty()),
        None => panic!("Expected a String, got None for completion text"),
    }
}

#[tokio::test]
async fn edit() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let resp = client
        .create_edit(|args| {
            args.input("What day of the week is it?")
                .instruction("Fix spelling mistakes")
                .n(1)
                .temperature(1.0)
        })
        .await
        .unwrap();

    dbg!(&resp);

    let text = resp.get_content(0).await;

    match text {
        Some(val) => assert!(!val.is_empty()),
        None => panic!("Expected a String, got None for edit text!"),
    }
}

#[tokio::test]
async fn image() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let resp = client
        .create_image(|args| {
            args.prompt("A realistic cat")
                .n(1)
                .size(ImageSize::Small)
                .response_format(ImageResponseFormat::Url)
        })
        .await
        .unwrap();

    let img = resp.get_content(0).await;

    match img {
        Some(val) => assert!(val.starts_with("https://")),
        None => panic!("Expected a String, got None for image url!"),
    }
}

#[tokio::test]
async fn models() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let models = client.get_models().await.unwrap();

    assert!(
        models
            .as_object()
            .unwrap()
            .get("data")
            .unwrap()
            .as_array()
            .unwrap()
            .len()
            > 60
    );
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
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let mut message1: HashMap<String, String> = HashMap::new();
    message1.insert("role".to_string(), "user".to_string());
    message1.insert(
        "content".to_string(),
        "Who won the world series in 2020?".to_string(),
    );

    let mut message2: HashMap<String, String> = HashMap::new();
    message2.insert("role".to_string(), "system".to_string());
    message2.insert(
        "content".to_string(),
        "You are a helpful assistant.".to_string(),
    );

    let messages: Vec<HashMap<String, String>> = vec![message1, message2];

    let resp = client
        .create_chat_completion(|args| args.messages(messages.clone()))
        .await;

    let resp = match resp {
        Ok(val) => val,
        Err(e) => panic!("An error occured while creating chat completion: {:?}", e),
    };

    let content = resp.get_content(0).await;

    match content {
        Some(val) => assert!(!val.is_empty()),
        None => panic!(
            "Expected a String, got None for chat completion content! Response: {:?}",
            resp.json
        ),
    }

    let second = client
        .create_chat_completion(|args| args.messages(messages))
        .await;

    let second = match second {
        Ok(val) => val,
        Err(e) => panic!("An error occured while creating chat completion: {:?}", e),
    };

    let content = second.get_content(0).await.unwrap();

    assert!(!content.is_empty());
}
