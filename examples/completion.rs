use openai_rs::{client::Client, args::CompletionArgs, response::Content};
use std::io::{stdin, stdout, Write};
use std::env;

#[tokio::main]
async fn main() {
    let mut prompt = String::new();

    print!("Enter a prompt: ");
    let _ = stdout().flush();

    stdin()
        .read_line(&mut prompt)
        .unwrap();

    let args = CompletionArgs::new(&prompt, None, None, None, None);

    let client = Client::new(env::var("OPENAI_API_KEY")
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