use openai_gpt_rs::models::CompletionModels;
use openai_gpt_rs::{client::Client, response::Content};
use std::env;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() {
    let mut prompt = String::new();

    print!("Enter a prompt: ");
    let _ = stdout().flush();

    stdin().read_line(&mut prompt).unwrap();

    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let resp = client
        .create_completion(|args| {
            args.prompt(prompt)
                .model(CompletionModels::TextDavinci3)
                .max_tokens(32)
                .n(1)
                .temperature(1.0)
        })
        .await
        .unwrap();

    let completion = resp.get_content(0).await.unwrap();

    println!("{}", completion);
}
