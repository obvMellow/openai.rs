use crate::args::*;
use crate::chat::Message;
use crate::client::Client;
use crate::models::CompletionModels;
use crate::response::*;
use std::env;

#[tokio::test]
async fn completion() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let resp = client
        .create_completion(|args| {
            args.prompt("Once upon a time")
                .model(CompletionModels::TextDavinci3)
                .max_tokens(32)
                .n(5)
                .temperature(1.0)
        })
        .await
        .unwrap();

    let text = resp.get_contents(0..5);

    for val in text {
        assert!(!val.is_empty());
    }
}

#[tokio::test]
async fn edit() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let resp = client
        .create_edit(|args| {
            args.input("What day of the week is it?")
                .instruction("Fix spelling mistakes")
                .n(5)
        })
        .await
        .unwrap();

    dbg!(&resp);

    let text = resp.get_contents(0..5);

    for val in text {
        assert!(!val.is_empty());
    }
}

#[tokio::test]
async fn image() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let resp = client
        .create_image(|args| {
            args.prompt("A realistic cat")
                .n(3)
                .size(ImageSize::Small)
                .response_format(ImageResponseFormat::Url)
        })
        .await
        .unwrap();

    let img = resp.get_contents(0..3);

    for val in img {
        assert!(!val.is_empty());
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

    let message1 = Message {
        role: "user".to_string(),
        content: "Who won the world series in 2020?".to_string(),
    };

    let message2 = Message {
        role: "system".to_string(),
        content: "You are a helpful assistant.".to_string(),
    };

    let messages = vec![message1, message2];

    let resp = client
        .create_chat_completion(|args| args.messages(messages.clone()))
        .await
        .expect("An error occured while creating chat completion");

    let content = resp.get_content(0).expect(
        format!(
            "Expected a String, got None for chat completion content! Response: {:?}",
            resp
        )
        .as_str(),
    );

    assert!(!content.is_empty());

    let second = client
        .create_chat_completion(|args| args.messages(messages).n(3))
        .await
        .expect("An error occured while creating chat completion");

    let content = second.get_contents(0..3);

    for val in content {
        assert!(!val.is_empty());
    }
}
