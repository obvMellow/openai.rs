use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION};
use reqwest::{Response, Error};

pub struct Args {
    pub prompt: String,
    pub model: String,
    pub suffix: String,
    pub max_tokens: u32,
    pub n: u32,
    pub temperature: u32,
}

impl Args {
    pub fn new(prompt: &str, max_tokens: Option<u32>, n: Option<u32>, suffix: Option<&str>, temperature: Option<u32>) -> Args {
        Args { prompt: prompt.to_string(),
            model: "gpt-3.5-turbo".to_string(),
            suffix: suffix.unwrap_or("").to_string(),
            max_tokens: max_tokens.unwrap_or(16),
            n: n.unwrap_or(1),
            temperature: temperature.unwrap_or(1) }
    }
}

pub struct Client {
    key: String
}

impl Client {
    pub fn new(key: &str) -> Client {
        Client { key: String::from(key) }
    }

    pub async fn create_completion(&self, args: Args) -> Result<Response, Error> {
        let client = reqwest::Client::new();

        let mut header = HeaderMap::new();
        header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        header.insert(AUTHORIZATION, self.key.parse().unwrap());

        client.post("https://api.openai.com/v1/completions")
            .headers(header)
            .body(format!("{{ prompt: {}, model: {}, max_tokens: {}, suffix: {}, n: {}, temperature: {} }}", args.prompt, args.model, args.max_tokens, args.suffix, args.n, args.temperature))
            .send()
            .await
    }
}
