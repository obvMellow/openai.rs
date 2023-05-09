use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::choice::*;
use crate::usage::Usage;

pub trait Content {
    fn get_content(&self, index: usize) -> Option<String>;

    fn get_contents<I>(&self, indices: I) -> Vec<String>
    where
        I: IntoIterator<Item = usize>,
    {
        indices
            .into_iter()
            .filter_map(|i| self.get_content(i))
            .collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompletionResp {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EditResp {
    pub object: String,
    pub created: u64,
    pub choices: Vec<EditChoice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageResp {
    pub created: u64,
    pub data: Vec<ImageData>,
}

#[derive(Debug)]
pub struct ChatResp {
    pub json: Value,
}

#[async_trait]
impl Content for CompletionResp {
    fn get_content(&self, index: usize) -> Option<String> {
        let content = self.choices.get(index)?.text.clone();
        Some(content)
    }
}

#[async_trait]
impl Content for EditResp {
    fn get_content(&self, index: usize) -> Option<String> {
        let content = self.choices.get(index)?.text.clone();
        Some(content)
    }
}

#[async_trait]
impl Content for ImageResp {
    fn get_content(&self, index: usize) -> Option<String> {
        let content = self.data.get(index)?.url.clone();
        Some(content)
    }
}

#[async_trait]
impl Content for ChatResp {
    fn get_content(&self, index: usize) -> Option<String> {
        let content = self
            .json
            .as_object()?
            .get("choices")?
            .as_array()?
            .get(index)?
            .as_object()?
            .get("message")?
            .as_object()?
            .get("content")?
            .as_str();
        content.map(|s| s.to_string())
    }
}
