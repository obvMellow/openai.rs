use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};
use reqwest::{Response, Error};
use serde_json::{json, Value};
use crate::args::{CompletionArgs, EditArgs};

pub struct Client {
    key: String
}

impl Client {
    pub fn new(key: &str) -> Client {
        Client { key: String::from(key) }
    }

    pub async fn create_completion(&self, args: CompletionArgs) -> Result<Response, Error> {
        let client = reqwest::Client::new();

        let mut header = HeaderMap::new();
        header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        header.insert(AUTHORIZATION, format!("Bearer {}", self.key).parse().unwrap());

        let body: Value = json!({
            "model": args.model,
            "prompt": args.prompt,
            "max_tokens": args.max_tokens,
            "temperature": args.temperature,
            "top_p": 1,
            "n": args.n,
            "stream": false,
            "logprobs": null
        });

        client.post("https://api.openai.com/v1/completions")
            .headers(header)
            .json(&body)
            .send()
            .await
    }

    pub async fn create_edit(&self, args: EditArgs) -> Result<Response, Error> {
        let client = reqwest::Client::new();

        let mut header = HeaderMap::new();
        header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        header.insert(AUTHORIZATION, format!("Bearer {}", self.key).parse().unwrap());

        let body = json!({
            "model": args.model,
            "input": args.input,
            "instruction": args.instruction,
            "n": args.n,
            "temperature": args.temperature,
            "top_p": args.top_p
        });

        client.post("https://api.openai.com/v1/edits")
        .headers(header)
        .body(body.to_string())
        .send()
        .await
    }
}
