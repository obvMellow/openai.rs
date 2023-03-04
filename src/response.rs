use reqwest::Response;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Content {
    async fn get_content(self, index: usize) -> Option<String>;
}

pub struct CompletionResp {
    pub resp: Response
}

pub struct EditResp {
    pub resp: Response
}

pub struct ImageResp {
    pub resp: Response
}

#[async_trait]
impl Content for CompletionResp {
    async fn get_content(self, index: usize) -> Option<String> {
        let resp = match self.resp.json::<Value>().await {
            Ok(val) => val,
            _ => Value::Null
        };

        let resp = match resp {
            Value::Object(val) => val,
            _ => return None
        };

        let resp = match resp.get("choices") {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.as_array() {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.get(index) {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.as_object() {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.get("text") {
            Some(val) => val,
            _ => return None
        };

        match resp.as_str() {
            Some(val) => return Some(val.to_string()),
            _ => return None
        }
    }
}

#[async_trait]
impl Content for EditResp {
    async fn get_content(self, index: usize) -> Option<String> {
        let resp = match self.resp.json::<Value>().await {
            Ok(val) => val,
            Err(_) => Value::Null
        };

        let resp = match resp {
            Value::Object(val) => val,
            _ => return None
        };

        let resp = match resp.get("choices") {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.as_array() {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.get(index) {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.as_object() {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.get("text") {
            Some(val) => val,
            _ => return None
        };

        match resp.as_str() {
            Some(val) => return Some(val.to_string()),
            _ => return None
        }
    }
}

#[async_trait]
impl Content for ImageResp {
    async fn get_content(self, index: usize) -> Option<String> {
        let resp = match self.resp.json::<Value>().await {
            Ok(val) => val,
            Err(_) => Value::Null
        };

        let resp = match resp {
            Value::Object(val) => val,
            _ => return None
        };

        let resp = match resp.get("data") {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.as_array() {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.get(index) {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.as_object() {
            Some(val) => val,
            _ => return None
        };

        let resp = match resp.get("url") {
            Some(val) => val,
            _ => return None
        };

        match resp.as_str() {
            Some(val) => return Some(val.to_string()),
            _ => return None
        }
    }
}