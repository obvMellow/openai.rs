use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: i32,
    pub model: String,
    pub choices: Vec<HashMap<String, String>>,
    pub usage: HashMap<String, i32>
}