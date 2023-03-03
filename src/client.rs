use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};
use reqwest::{Response, Error};
use serde::{Serialize};
use reqwest::Client as HttpClient;
use serde_json::{json, Value};
use crate::args::{CompletionArgs, EditArgs, ImageArgs, ImageResponseFormat, ImageSize};

pub struct Client {
    client: HttpClient,
    api_key: String,
    header: HeaderMap
}

impl Client {
    pub fn new(key: &str) -> Client {
        let mut header = HeaderMap::new();
        header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        header.insert(AUTHORIZATION, format!("Bearer {}", key).parse().unwrap());

        Client { client: HttpClient::new(), api_key: String::from(key), header }
    }

    pub async fn create_completion(&self, args: &CompletionArgs) -> Result<Response, Error> {
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

        self.client.post("https://api.openai.com/v1/completions")
            .headers(self.header.clone())
            .json(&body)
            .send()
            .await
    }

    pub async fn create_edit(&self, args: &EditArgs) -> Result<Response, Error> {
        let body = json!({
            "model": args.model,
            "input": args.input,
            "instruction": args.instruction,
            "n": args.n,
            "temperature": args.temperature,
            "top_p": args.top_p
        });

        self.client.post("https://api.openai.com/v1/edits")
        .headers(self.header.clone())
        .body(body.to_string())
        .send()
        .await
    }

    pub async fn create_image(&self, args: &ImageArgs) -> Result<Response, Error> {
        let body = json!({
            "model": "image-alpha-001",
            "prompt": args.prompt,
            "n": args.n,
            "size": args.size,
            "response_format": args.response_format
        });

        self.client.post("https://api.openai.com/v1/images/generations")
            .headers(self.header.clone())
            .body(body.to_string())
            .send()
            .await
    }
}
