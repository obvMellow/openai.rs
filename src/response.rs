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
        get_content_helper(self.resp, index, "choices", "text").await
    }
}

#[async_trait]
impl Content for EditResp {
    async fn get_content(self, index: usize) -> Option<String> {
        get_content_helper(self.resp, index, "choices", "text").await
    }
}

#[async_trait]
impl Content for ImageResp {
    async fn get_content(self, index: usize) -> Option<String> {
        get_content_helper(self.resp, index, "data", "url").await
    }
}

async fn get_content_helper(resp: Response, index: usize, arr: &str, txt: &str) -> Option<String> {
    let resp = match resp.json::<Value>().await {
        Ok(val) => val,
        _ => Value::Null
    };

    let resp = match resp {
        Value::Object(val) => val,
        _ => return None
    };

    let resp = match resp.get(arr) {
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

    let resp = match resp.get(txt) {
        Some(val) => val,
        _ => return None
    };

    match resp.as_str() {
        Some(val) => return Some(val.to_string()),
        _ => return None
    }
}