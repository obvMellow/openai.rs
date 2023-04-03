use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Content {
    async fn get_content(&self, index: usize) -> Option<String>;
}

#[derive(Debug)]
pub struct CompletionResp {
    pub json: Value,
}

#[derive(Debug)]
pub struct EditResp {
    pub json: Value,
}

#[derive(Debug)]
pub struct ImageResp {
    pub json: Value,
}

#[derive(Debug)]
pub struct ChatResp {
    pub json: Value,
}

#[async_trait]
impl Content for CompletionResp {
    async fn get_content(&self, index: usize) -> Option<String> {
        let content = self
            .json
            .as_object()?
            .get("choices")?
            .as_array()?
            .get(index)?
            .as_object()?
            .get("text")?
            .as_str();
        content.map(|s| s.to_string())
    }
}

#[async_trait]
impl Content for EditResp {
    async fn get_content(&self, index: usize) -> Option<String> {
        let content = self
            .json
            .as_object()?
            .get("choices")?
            .as_array()?
            .get(index)?
            .as_object()?
            .get("text")?
            .as_str();
        content.map(|s| s.to_string())
    }
}

#[async_trait]
impl Content for ImageResp {
    async fn get_content(&self, index: usize) -> Option<String> {
        let content = self
            .json
            .as_object()?
            .get("data")?
            .as_array()?
            .get(index)?
            .as_object()?
            .get("url")?
            .as_str();
        content.map(|s| s.to_string())
    }
}

#[async_trait]
impl Content for ChatResp {
    async fn get_content(&self, index: usize) -> Option<String> {
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
