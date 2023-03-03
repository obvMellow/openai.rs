pub struct CompletionArgs {
    pub prompt: String,
    pub model: String,
    pub suffix: String,
    pub max_tokens: u32,
    pub n: u32,
    pub temperature: f64,
}

impl CompletionArgs {
    pub fn new(prompt: &str, max_tokens: Option<u32>, n: Option<u32>, suffix: Option<&str>, temperature: Option<f64>) -> CompletionArgs {
        CompletionArgs { prompt: prompt.to_string(),
            model: "text-davinci-003".to_string(),
            suffix: suffix.unwrap_or("").to_string(),
            max_tokens: max_tokens.unwrap_or(16),
            n: n.unwrap_or(1),
            temperature: temperature.unwrap_or(1.0) }
    }
}

pub struct EditArgs {
    pub model: String,
    pub input: String,
    pub instruction: String,
    pub n: i32,
    pub temperature: f64,
    pub top_p: f64
}

impl EditArgs {
    pub fn new(model: Option<String>, instruction: String, input: Option<String>, n: Option<i32>, temperature: Option<f64>, top_p: Option<f64>) -> EditArgs {
        EditArgs { model: model.unwrap_or("text-davinci-edit-001".to_string()),
            input: input.unwrap_or("".to_string()),
            instruction,
            n: n.unwrap_or(1),
            temperature: temperature.unwrap_or(1.0),
            top_p: top_p.unwrap_or(1.0) }
    }
}

pub enum ImageSize {
    Small,
    Medium,
    Big
}

pub enum ImageResponseFormat {
    Url,
    B64Json
}

pub struct ImageArgs {
    pub prompt: String,
    pub n: i32,
    pub size: String,
    pub response_format: String
}

impl ImageArgs {
    pub fn new(prompt: String, n: Option<i32>, size: Option<ImageSize>, response_format: Option<ImageResponseFormat>) -> ImageArgs {
        let size = match size {
            Some(val) => {
                match val {
                    ImageSize::Small => "256x256".to_string(),
                    ImageSize::Medium => "512x512".to_string(),
                    ImageSize::Big => "1024x1024".to_string()
                }
            }
            _ => "1024x1024".to_string()
        };

        let response_format = match response_format {
            Some(val) => match val {
                ImageResponseFormat::Url => "url".to_string(),
                ImageResponseFormat::B64Json => "b64_json".to_string()
            }
            _ => "url".to_string()
        };

        ImageArgs { prompt, n: n.unwrap_or(1), size, response_format }
    }
}