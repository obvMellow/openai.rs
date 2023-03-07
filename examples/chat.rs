use openai_rs::{args::ChatArgs, client::Client, response::Content};
use std::io::{stdin, stdout, Write};
use std::env;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let client = Client::new(env::var("OPENAI_API_KEY")
        .unwrap()
        .as_str());

    let mut role: String;
    let mut message = String::new();

    let mut index = String::new();

    print!("    1: system\n    2: assistant\n    3: user\nSelect a role: ");
    let _ = stdout().flush();

    stdin()
        .read_line(&mut index)
        .unwrap();

    if index.trim() == "1" {
        role = "system".to_string();
    }
    else if index.trim() == "2" {
        role = "assistant".to_string();
    }
    else if index.trim() == "3" {
        role = "user".to_string();
    }
    else {
        panic!("Invalid role!");
    }

    role = role.trim().to_string();

    print!("Enter a message: ");
    let _ = stdout().flush();

    stdin()
        .read_line(&mut message)
        .unwrap();

    let message = message.trim().to_string();

    let mut messages = HashMap::new();
    messages.insert("role".to_string(), role);
    messages.insert("content".to_string(), message);

    let messages: Vec<HashMap<String, String>> = vec![messages];

    let args = ChatArgs::new(messages, None, None, None, None, None, None);

    let resp = client.create_chat_completion(&args).await.unwrap();

    let json = resp.get_json().await.unwrap();

    let content = json.as_object()
            .unwrap()
            .get("choices");

    let content = match content {
        Some(val) => val
            .as_array()
            .unwrap()
            .get(0)
            .unwrap()
            .as_object()
            .unwrap()
            .get("message")
            .unwrap()
            .as_object()
            .unwrap()
            .get("content")
            .unwrap()
            .as_str()
            .map(|s| s.to_string())
            .unwrap(),
        None => panic!("An error occured while creating chat completion: {:?}", json.as_object().unwrap())
    };

    println!("Response: {}", content);
}