use crate::args::*;
use crate::error::ResponseError;
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
    /// Creates a new client with the given api key.
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
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a mutable reference to `CompletionArgs` and returns it.
    ///
    /// # Example
    ///
    /// ```
    /// use openai_gpt_rs::{args::CompletionArgs, client::Client, response::{CompletionResp, Content}, models::CompletionModels};
    /// use std::env;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());
    ///
    ///     let resp = client.create_completion(|args| {
    ///                     args.prompt("This is a test")
    ///                         .model(CompletionModels::TextDavinci3)
    ///                         .max_tokens(32)
    ///                         .n(5)
    ///                })
    ///            .await
    ///           .unwrap();
    ///
    ///     let text = resp.get_contents(0..5);
    ///
    ///     for val in text {
    ///        assert!(!val.is_empty());
    ///    }
    /// }
    ///
    /// ```
    ///
    /// # Panics
    /// This function will panic if the request to OpenAI fails.
    ///
    pub async fn create_completion<T>(&self, f: T) -> Result<CompletionResp, ResponseError>
    where
        T: FnOnce(&mut CompletionArgs) -> &mut CompletionArgs,
    {
        let mut args = CompletionArgs::default();
        f(&mut args);

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
            .await
            .unwrap();

        let json: Value = resp.json().await.unwrap();

        if let Some(e) = json.as_object().unwrap().get("error") {
            return Err(serde_json::from_value(e.clone()).unwrap());
        }

        Ok(serde_json::from_value(json).unwrap())
    }

    /// Makes an api call to OpenAI Edit API and returns the response.
    /// 
    /// # Arguments
    /// 
    /// * `f` - A closure that takes a mutable reference to `EditArgs` and returns it.
    /// 
    /// # Example
    /// ```
    /// use openai_gpt_rs::{args::EditArgs, client::Client, response::{EditResp, Content}, models::EditModels};
    /// use std::env;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());
    ///
    ///     let resp = client.create_edit(|args| {
    ///                     args.model(EditModels::TextDavinciEdit1)
    ///                         .input("How is you dae")
    ///                         .instruction("Fix the spelling mistakes")
    ///                         .n(5)
    ///                 })
    ///                .await
    ///                .unwrap();
    ///
    ///     let text = resp.get_contents(0..5);
    ///
    ///     for val in text {
    ///         assert!(!val.is_empty());
    ///     }
    /// }
    /// ```
    /// # Panics
    /// This function will panic if the request to OpenAI fails.
    ///
    pub async fn create_edit<T>(&self, f: T) -> Result<EditResp, ResponseError>
    where
        T: FnOnce(&mut EditArgs) -> &mut EditArgs,
    {
        let mut args = EditArgs::default();
        f(&mut args);

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
            .await
            .unwrap();

        let json: Value = resp.json().await.unwrap();

        if let Some(e) = json.as_object().unwrap().get("error") {
            return Err(serde_json::from_value(e.clone()).unwrap());
        }

        Ok(serde_json::from_value(json).unwrap())
    }

    /// Makes an api call to OpenAI Image API and returns the response.
    /// 
    /// # Arguments
    /// 
    /// * `f` - A closure that takes a mutable reference to `ImageArgs` and returns it.
    /// 
    /// # Example
    /// ```
    /// use openai_gpt_rs::{args::{ImageArgs, ImageSize}, client::Client, response::{ImageResp, Content}};
    /// use std::env;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());
    ///     
    ///     let resp = client.create_image(|args| {
    ///                         args.prompt("Kitty")
    ///                             .size(ImageSize::Small)
    ///                             .n(2)
    ///                     })
    ///                     .await
    ///                     .unwrap();
    ///
    ///     let urls = resp.get_contents(0..2);
    ///
    ///     for val in urls {
    ///         assert!(!val.is_empty());
    ///     }
    /// }
    /// ```
    /// # Panics
    /// This function will panic if the request to OpenAI fails.
    ///
    pub async fn create_image<T>(&self, f: T) -> Result<ImageResp, ResponseError>
    where
        T: FnOnce(&mut ImageArgs) -> &mut ImageArgs,
    {
        let mut args = ImageArgs::default();
        f(&mut args);

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
            .await
            .unwrap();

        let json: Value = resp.json().await.unwrap();

        if let Some(e) = json.as_object().unwrap().get("error") {
            return Err(serde_json::from_value(e.clone()).unwrap());
        }

        Ok(serde_json::from_value(json).unwrap())
    }

    /// Returns the client's api key.
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

    /// Makes an api call to OpenAI Chat Completion API and returns the response.
    /// 
    /// # Arguments
    /// 
    /// * `f` - A closure that takes a mutable reference to `ChatArgs` and returns it.
    /// 
    /// # Example
    /// ```
    /// use openai_gpt_rs::{args::ChatArgs, client::Client, response::{ChatResp, Content}, models::ChatModels};
    /// use std::env;
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(env::var("OPENAI_API_KEY").unwrap().as_str());
    ///
    ///     let mut message1: HashMap<String, String> = HashMap::new();
    ///     message1.insert("role".to_string(), "user".to_string());
    ///     message1.insert(
    ///         "content".to_string(),
    ///         "Who won the world series in 2020?".to_string(),
    ///     );
    ///
    ///     let mut message2: HashMap<String, String> = HashMap::new();
    ///     message2.insert("role".to_string(), "system".to_string());
    ///     message2.insert(
    ///         "content".to_string(),
    ///         "You are a helpful assistant.".to_string(),
    ///     );
    ///
    ///     let messages: Vec<HashMap<String, String>> = vec![message1, message2];
    ///
    ///     let resp = client
    ///         .create_chat_completion(|args| args.messages(messages.clone()))
    ///         .await
    ///         .unwrap();
    ///
    ///     let contents = resp.get_content(0).unwrap();
    ///
    ///     assert!(!contents.is_empty());
    /// }
    /// ```
    /// # Errors
    /// This function will return an error if the api call fails.
    /// The error will be of type `reqwest::Error`.
    ///     
    pub async fn create_chat_completion<T>(&self, f: T) -> Result<ChatResp, Error>
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
        f(&mut args);

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
