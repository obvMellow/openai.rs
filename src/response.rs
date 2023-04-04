use async_trait::async_trait;
use serde_json::Value;

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
    fn get_content(&self, index: usize) -> Option<String> {
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
    fn get_content(&self, index: usize) -> Option<String> {
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
    fn get_content(&self, index: usize) -> Option<String> {
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
