use crate::args::*;
use crate::response::*;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client as HttpClient;
use reqwest::Error;
use serde_json::{json, Value};

pub struct Client {
    client: HttpClient,
    api_key: String,
    header: HeaderMap,
}

impl Client {
    pub fn new(key: &str) -> Client {
        let mut header = HeaderMap::new();
        header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        header.insert(AUTHORIZATION, format!("Bearer {}", key).parse().unwrap());

        Client {
            client: HttpClient::new(),
            api_key: String::from(key),
            header,
        }
    }

    /// Makes an api call to OpenAI Completion API and returns the response.
    pub async fn create_completion<T>(&self, arg: T) -> Result<CompletionResp, Error>
    where
        T: FnOnce(&mut CompletionArgs) -> &mut CompletionArgs,
    {
        let mut args = CompletionArgs::default();
        arg(&mut args);

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

        let resp = self
            .client
            .post("https://api.openai.com/v1/completions")
            .headers(self.header.clone())
            .json(&body)
            .send()
            .await;

        match resp {
            Ok(val) => Ok(CompletionResp {
                json: val.json().await?,
            }),
            Err(e) => Err(e),
        }
    }

    /// Makes an api call to OpenAI Edit API and returns the response.
    pub async fn create_edit<T>(&self, arg: T) -> Result<EditResp, Error>
    where
        T: FnOnce(&mut EditArgs) -> &mut EditArgs,
    {
        let mut args = EditArgs::default();
        arg(&mut args);

        let body = json!({
            "model": args.model,
            "input": args.input,
            "instruction": args.instruction,
            "n": args.n,
            "temperature": args.temperature,
            "top_p": args.top_p
        });

        let resp = self
            .client
            .post("https://api.openai.com/v1/edits")
            .headers(self.header.clone())
            .body(body.to_string())
            .send()
            .await;

        match resp {
            Ok(val) => Ok(EditResp {
                json: val.json().await?,
            }),
            Err(e) => Err(e),
        }
    }

    /// Makes an api call to OpenAI Image API and returns the response.
    pub async fn create_image<T>(&self, arg: T) -> Result<ImageResp, Error>
    where
        T: FnOnce(&mut ImageArgs) -> &mut ImageArgs,
    {
        let mut args = ImageArgs::default();
        arg(&mut args);

        let body = json!({
            "model": "image-alpha-001",
            "prompt": args.prompt,
            "n": args.n,
            "size": args.size,
            "response_format": args.response_format
        });

        let resp = self
            .client
            .post("https://api.openai.com/v1/images/generations")
            .headers(self.header.clone())
            .body(body.to_string())
            .send()
            .await;

        match resp {
            Ok(val) => Ok(ImageResp {
                json: val.json().await?,
            }),
            Err(e) => Err(e),
        }
    }

    /// Returns a reference to the client's api key.
    pub fn get_key(&self) -> &String {
        &self.api_key
    }

    /// Sets the client's api key to the value of given key.
    pub fn set_key(&mut self, new_key: &str) {
        self.api_key = new_key.to_string();
    }

    /// Returns a json listing all the models
    pub async fn get_models(&self) -> Result<Value, Error> {
        let resp = self
            .client
            .get("https://api.openai.com/v1/models")
            .headers(self.header.clone())
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(resp)
    }

    pub async fn create_chat_completion<T>(&self, arg: T) -> Result<ChatResp, Error>
    where
        T: FnOnce(&mut ChatArgs) -> &mut ChatArgs,
    {
        let mut args = ChatArgs {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![],
            n: 1,
            temperature: 1.0,
            top_p: 1.0,
            max_tokens: 32,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
        };
        arg(&mut args);

        let body = json!({
        "model": args.model,
        "messages": args.messages,
        "max_tokens": args.max_tokens,
        "temperature": args.temperature,
        "top_p": args.top_p,
        "n": args.n,
        "presence_penalty": args.presence_penalty,
        "frequency_penalty": args.frequency_penalty
        });

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(self.header.clone())
            .json(&body)
            .send()
            .await?;

        Ok(ChatResp {
            json: resp.json().await?,
        })
    }
}
