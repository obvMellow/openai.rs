use openai_gpt_rs::models::EditModels;
use openai_gpt_rs::{client::Client, response::Content};
use std::env;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() {
    let mut prompt = String::new();
    let mut instruction = String::new();

    print!("Enter a prompt: ");
    let _ = stdout().flush();

    stdin().read_line(&mut prompt).unwrap();

    print!("Enter the instruction: ");
    let _ = stdout().flush();

    stdin().read_line(&mut instruction).unwrap();

    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let resp = client
        .create_edit(|args| {
            args.input(prompt)
                .instruction(instruction)
                .model(EditModels::TextDavinciEdit1)
                .n(1)
                .temperature(1.0)
        })
        .await
        .unwrap();

    let text = resp.get_content(0).unwrap();

    println!("{}", text);
}
