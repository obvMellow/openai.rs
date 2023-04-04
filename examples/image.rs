use openai_gpt_rs::{args::ImageSize, client::Client, response::Content};
use std::env;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() {
    let mut prompt = String::new();

    print!("Enter a prompt: ");
    let _ = stdout().flush();

    stdin().read_line(&mut prompt).unwrap();

    println!("Generating image...\n");

    let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());

    let resp = client
        .create_image(|args| args.prompt(prompt).size(ImageSize::Medium).n(1))
        .await
        .unwrap();

    let url = resp.get_content(0).unwrap();

    println!("Url: {}", url);
}
