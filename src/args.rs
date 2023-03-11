use std::collections::HashMap;

#[derive(Debug)]
pub struct CompletionArgs {
    pub prompt: String,
    pub model: String,
    pub suffix: String,
    pub max_tokens: u32,
    pub n: u32,
    pub temperature: f64,
}

impl CompletionArgs {
    pub fn new(
        prompt: &str,
        max_tokens: Option<u32>,
        n: Option<u32>,
        suffix: Option<&str>,
        temperature: Option<f64>,
    ) -> CompletionArgs {
        CompletionArgs {
            prompt: prompt.to_string(),
            model: "text-davinci-003".to_string(),
            suffix: suffix.unwrap_or("").to_string(),
            max_tokens: max_tokens.unwrap_or(16),
            n: n.unwrap_or(1),
            temperature: temperature.unwrap_or(1.0),
        }
    }
}

#[derive(Debug)]
pub struct EditArgs {
    pub model: String,
    pub input: String,
    pub instruction: String,
    pub n: i32,
    pub temperature: f64,
    pub top_p: f64,
}

impl EditArgs {
    pub fn new(
        model: Option<&str>,
        instruction: &str,
        input: &str,
        n: Option<i32>,
        temperature: Option<f64>,
        top_p: Option<f64>,
    ) -> EditArgs {
        EditArgs {
            model: model.unwrap_or("text-davinci-edit-001").to_string(),
            input: input.to_string(),
            instruction: instruction.to_string(),
            n: n.unwrap_or(1),
            temperature: temperature.unwrap_or(1.0),
            top_p: top_p.unwrap_or(1.0),
        }
    }
}

pub enum ImageSize {
    Small,
    Medium,
    Big,
}

impl std::fmt::Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Small => "256x256",
            Self::Medium => "512x512",
            Self::Big => "1024x1024"
        };

        write!(f, "{}", str)
    }
}

pub enum ImageResponseFormat {
    Url,
    B64Json,
}

#[derive(Debug)]
pub struct ImageArgs {
    pub prompt: String,
    pub n: i32,
    pub size: String,
    pub response_format: String,
}

impl ImageArgs {
    pub fn new(
        prompt: &str,
        n: Option<i32>,
        size: Option<ImageSize>,
        response_format: Option<ImageResponseFormat>,
    ) -> ImageArgs {
        let size = match size {
            Some(val) => match val {
                ImageSize::Small => "256x256".to_string(),
                ImageSize::Medium => "512x512".to_string(),
                ImageSize::Big => "1024x1024".to_string(),
            },
            _ => "1024x1024".to_string(),
        };

        let response_format = match response_format {
            Some(val) => match val {
                ImageResponseFormat::Url => "url".to_string(),
                ImageResponseFormat::B64Json => "b64_json".to_string(),
            },
            _ => "url".to_string(),
        };

        ImageArgs {
            prompt: prompt.to_string(),
            n: n.unwrap_or(1),
            size,
            response_format,
        }
    }
}

#[derive(Debug)]
pub struct ChatArgs {
    pub model: String,
    pub messages: Vec<HashMap<String, String>>,
    pub n: i32,
    pub temperature: f64,
    pub top_p: f64,
    pub max_tokens: u32,
    pub presence_penalty: f64,
    pub frequency_penalty: f64,
}

impl ChatArgs {
    pub fn new(
        messages: Vec<HashMap<String, String>>,
        max_tokens: Option<u32>,
        n: Option<i32>,
        temperature: Option<f64>,
        top_p: Option<f64>,
        presence_penalty: Option<f64>,
        frequency_penalty: Option<f64>,
    ) -> ChatArgs {
        ChatArgs {
            model: "gpt-3.5-turbo".to_string(),
            messages,
            n: n.unwrap_or(1),
            temperature: temperature.unwrap_or(1.0),
            top_p: top_p.unwrap_or(1.0),
            max_tokens: max_tokens.unwrap_or(2048),
            presence_penalty: presence_penalty.unwrap_or(0.0),
            frequency_penalty: frequency_penalty.unwrap_or(0.0),
        }
    }
}
