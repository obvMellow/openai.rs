use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};
use reqwest::Error;
use reqwest::Client as HttpClient;
use serde_json::{json, Value};
use crate::args::{CompletionArgs, EditArgs, ImageArgs};
use crate::response::{CompletionResp, EditResp, ImageResp};

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

    /// Makes an api call to OpenAI Completion API and returns the response.
    pub async fn create_completion(&self, args: &CompletionArgs) -> Result<CompletionResp, Error> {
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

        let resp = self.client.post("https://api.openai.com/v1/completions")
            .headers(self.header.clone())
            .json(&body)
            .send()
            .await;

        match resp {
            Ok(val) => return Ok(CompletionResp { resp: val }),
            Err(e) => return Err(e)
        }
    }

    /// Makes an api call to OpenAI Edit API and returns the response.
    pub async fn create_edit(&self, args: &EditArgs) -> Result<EditResp, Error> {
        let body = json!({
            "model": args.model,
            "input": args.input,
            "instruction": args.instruction,
            "n": args.n,
            "temperature": args.temperature,
            "top_p": args.top_p
        });

        let resp = self.client.post("https://api.openai.com/v1/edits")
        .headers(self.header.clone())
        .body(body.to_string())
        .send()
        .await;

        match resp {
            Ok(val) => return Ok(EditResp { resp: val }),
            Err(e) => return Err(e)
        }
    }

    /// Makes an api call to OpenAI Image API and returns the response.
    pub async fn create_image(&self, args: &ImageArgs) -> Result<ImageResp, Error> {
        let body = json!({
            "model": "image-alpha-001",
            "prompt": args.prompt,
            "n": args.n,
            "size": args.size,
            "response_format": args.response_format
        });

        let resp = self.client.post("https://api.openai.com/v1/images/generations")
            .headers(self.header.clone())
            .body(body.to_string())
            .send()
            .await;

        match resp {
            Ok(val) => return Ok(ImageResp { resp: val }),
            Err(e) => return Err(e)
        }
    }

    /// Returns a reference to the client's api key.
    pub fn key(&self) -> &String {
        return &self.api_key;
    }
}
