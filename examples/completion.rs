use openai::{client::Client, args::CompletionArgs, response::Content};
use std::io::{stdin, stdout, Write};
use std::fs;

#[tokio::main]
async fn main() {
    let mut prompt = String::new();

    print!("Enter a prompt: ");
    let _ = stdout().flush();

    stdin()
        .read_line(&mut prompt)
        .unwrap();

    let args = CompletionArgs::new(prompt.as_str(), None, None, None, None);

    let client = Client::new(fs::read_to_string("key.txt")
        .unwrap()
        .as_str());

    let resp = client.create_completion(&args)
        .await
        .unwrap();

    let completion = resp.get_content(0)
        .await
        .unwrap();

    println!("{}", completion);
}