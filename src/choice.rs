use serde::{Deserialize, Serialize};

use crate::chat::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionChoice {
    pub text: String,
    pub index: usize,
    pub logprobs: Option<u8>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditChoice {
    pub text: String,
    pub index: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageData {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatChoice {
    pub index: usize,
    pub message: Message,
    pub finish_reason: String,
}
