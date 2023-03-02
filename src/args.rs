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