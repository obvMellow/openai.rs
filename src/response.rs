use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: i32,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage
}

#[derive(Deserialize, Serialize)]
pub struct Choice {
    pub text: String,
    pub index: i32,
    pub logprobs: bool,
    pub finish_reason: String
}

#[derive(Deserialize, Serialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}