use openai_rs::{args::{ImageArgs, ImageSize}, response::Content, client::Client};
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

    println!("Generating image...\n");

    let args = ImageArgs::new(&prompt, Some(1), Some(ImageSize::Big), None);

    let client = Client::new(fs::read_to_string("key.txt")
        .unwrap()
        .as_str());

    let resp = client.create_image(&args).await.unwrap();

    let url = resp.get_content(0).await.unwrap();

    println!("Url: {}", url);
}