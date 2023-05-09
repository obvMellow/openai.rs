use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseError {
    pub code: Option<u16>,
    pub message: String,
    pub param: Option<String>,
    pub r#type: String,
}
