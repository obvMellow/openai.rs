use reqwest::Response;
use reqwest::Error;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Content {
    async fn get_content(self, index: usize) -> Option<String>;

    async fn get_json(self) -> Result<Value, Error>;
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

pub struct ChatResp {
    pub resp: Response
}

#[async_trait]
impl Content for CompletionResp {
    async fn get_content(self, index: usize) -> Option<String> {
        get_content_helper(self.resp, index, "choices", "text").await
    }

    async fn get_json(self) -> Result<Value, Error> {
        self.resp.json::<Value>()
            .await
    }
}

#[async_trait]
impl Content for EditResp {
    async fn get_content(self, index: usize) -> Option<String> {
        get_content_helper(self.resp, index, "choices", "text").await
    }

    async fn get_json(self) -> Result<Value, Error> {
        self.resp.json::<Value>()
            .await
    }
}

#[async_trait]
impl Content for ImageResp {
    async fn get_content(self, index: usize) -> Option<String> {
        get_content_helper(self.resp, index, "data", "url").await
    }

    async fn get_json(self) -> Result<Value, Error> {
        self.resp.json::<Value>()
            .await
    }
}

#[async_trait]
impl Content for ChatResp {
    /// Keep in mind that this implementation will only give the content, not the role.
    async fn get_content(self, index: usize) -> Option<String> {
        self.resp.json::<Value>()
            .await.ok()?
            .as_object()?
            .get("choices")?
            .as_array()?
            .get(index)?
            .as_object()?
            .get("message")?
            .as_object()?
            .get("content")?
            .as_str()
            .map(|s| s.to_string())
    }

    async fn get_json(self) -> Result<Value, Error> {
        self.resp.json::<Value>()
            .await
    }
}

async fn get_content_helper(resp: Response, index: usize, arr: &str, txt: &str) -> Option<String> {
    resp.json::<Value>()
        .await.ok()?
        .as_object()?
        .get(arr)?
        .as_array()?
        .get(index)?
        .as_object()?
        .get(txt)?
        .as_str()
        .map(|s| s.to_string())
}