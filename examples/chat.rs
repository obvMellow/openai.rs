use openai_gpt_rs::{client::Client, response::Content};
use std::collections::HashMap;
use std::env;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() {
    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let mut role: String;
    let mut message = String::new();

    let mut index = String::new();

    print!("    1: system\n    2: assistant\n    3: user\nSelect a role: ");
    let _ = stdout().flush();

    stdin().read_line(&mut index).unwrap();

    if index.trim() == "1" {
        role = "system".to_string();
    } else if index.trim() == "2" {
        role = "assistant".to_string();
    } else if index.trim() == "3" {
        role = "user".to_string();
    } else {
        panic!("Invalid role!");
    }

    role = role.trim().to_string();

    print!("Enter a message: ");
    let _ = stdout().flush();

    stdin().read_line(&mut message).unwrap();

    let message = message.trim().to_string();

    let mut messages = HashMap::new();
    messages.insert("role".to_string(), role);
    messages.insert("content".to_string(), message);

    let messages: Vec<HashMap<String, String>> = vec![messages];

    let resp = client
        .create_chat_completion(|args| args.messages(messages))
        .await
        .unwrap();

    let content = resp.get_content(0).await.unwrap();

    println!("Response: {}", content);
}
