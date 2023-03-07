use openai_rs::{args::EditArgs, client::Client, response::Content};
use std::io::{stdin, stdout, Write};
use std::env;

#[tokio::main]
async fn main() {
    let mut prompt = String::new();
    let mut instruction = String::new();

    print!("Enter a prompt: ");
    let _ = stdout().flush();

    stdin()
        .read_line(&mut prompt)
        .unwrap();

    print!("Enter the instruction: ");
    let _ = stdout().flush();

    stdin()
        .read_line(&mut instruction)
        .unwrap();

    let args = EditArgs::new(None,
        &instruction,
        &prompt,
        None,
        None,
        None);

    let client = Client::new(env::var("OPENAI_API_KEY")
        .unwrap()
        .as_str());

    let resp = client.create_edit(&args).await.unwrap();

    let text = resp.get_content(0).await.unwrap();

    println!("{}", text);
}