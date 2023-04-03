use std::collections::HashMap;

#[derive(Debug)]
pub struct CompletionArgs {
    pub prompt: String,
    pub model: String,
    pub suffix: String,
    pub max_tokens: u32,
    pub n: usize,
    pub temperature: f64,
}

impl CompletionArgs {
    /// Create a new CompletionArgs struct with default values
    pub fn default() -> Self {
        Self {
            prompt: "".to_string(),
            model: "text-davinci-003".to_string(),
            suffix: "".to_string(),
            max_tokens: 32,
            n: 1,
            temperature: 1.0,
        }
    }

    /// Set the prompt for the completion
    pub fn prompt<T>(&mut self, prompt: T) -> &mut Self
    where
        T: ToString,
    {
        self.prompt = prompt.to_string();
        self
    }

    /// Set the model to use for the completion
    pub fn model<T>(&mut self, model: T) -> &mut Self
    where
        T: ToString,
    {
        self.model = model.to_string();
        self
    }

    /// Set the suffix for the completion
    pub fn suffix<T>(&mut self, suffix: T) -> &mut Self
    where
        T: ToString,
    {
        self.suffix = suffix.to_string();
        self
    }

    /// Set the maximum number of tokens for the completion
    pub fn max_tokens(&mut self, max_tokens: u32) -> &mut Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Set the number of completions to return
    pub fn n(&mut self, n: usize) -> &mut Self {
        self.n = n;
        self
    }

    /// Set the temperature for the completion
    pub fn temperature(&mut self, temperature: f64) -> &mut Self {
        self.temperature = temperature;
        self
    }
}

#[derive(Debug)]
pub struct EditArgs {
    pub model: String,
    pub input: String,
    pub instruction: String,
    pub n: usize,
    pub temperature: f64,
    pub top_p: f64,
}

impl EditArgs {
    /// Create a new EditArgs struct with default values
    pub fn default() -> Self {
        Self {
            model: "text-davinci-edit-001".to_string(),
            input: "".to_string(),
            instruction: "".to_string(),
            n: 1,
            temperature: 1.0,
            top_p: 1.0,
        }
    }

    /// Set the model to use for the edit
    pub fn model<T>(&mut self, model: T) -> &mut Self
    where
        T: ToString,
    {
        self.model = model.to_string();
        self
    }

    /// Set the input for the edit
    pub fn input<T>(&mut self, input: T) -> &mut Self
    where
        T: ToString,
    {
        self.input = input.to_string();
        self
    }

    /// Set the instruction for the edit
    pub fn instruction<T>(&mut self, instruction: T) -> &mut Self
    where
        T: ToString,
    {
        self.instruction = instruction.to_string();
        self
    }

    /// Set the number of edits to return
    pub fn n(&mut self, n: usize) -> &mut Self {
        self.n = n;
        self
    }

    /// Set the temperature for the edit
    pub fn temperature(&mut self, temperature: f64) -> &mut Self {
        self.temperature = temperature;
        self
    }

    /// Set the top_p for the edit
    pub fn top_p(&mut self, top_p: f64) -> &mut Self {
        self.top_p = top_p;
        self
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
            Self::Big => "1024x1024",
        };

        write!(f, "{}", str)
    }
}

impl ImageSize {
    pub fn to_string(&self) -> String {
        match self {
            Self::Small => "256x256".to_string(),
            Self::Medium => "512x512".to_string(),
            Self::Big => "1024x1024".to_string(),
        }
    }
}

pub enum ImageResponseFormat {
    Url,
    B64Json,
}

impl ImageResponseFormat {
    pub fn to_string(&self) -> String {
        match self {
            Self::Url => "url".to_string(),
            Self::B64Json => "b64json".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ImageArgs {
    pub prompt: String,
    pub n: usize,
    pub size: String,
    pub response_format: String,
}

impl ImageArgs {
    /// Create a new ImageArgs struct with default values
    pub fn default() -> Self {
        Self {
            prompt: "".to_string(),
            n: 1,
            size: ImageSize::Medium.to_string(),
            response_format: ImageResponseFormat::Url.to_string(),
        }
    }

    /// Set the prompt for the image
    pub fn prompt<T>(&mut self, prompt: T) -> &mut Self
    where
        T: ToString,
    {
        self.prompt = prompt.to_string();
        self
    }

    /// Set the number of images to return
    pub fn n(&mut self, n: usize) -> &mut Self {
        self.n = n;
        self
    }

    /// Set the size of the images to return
    pub fn size(&mut self, size: ImageSize) -> &mut Self {
        self.size = size.to_string();
        self
    }

    /// Set the response format for the images to return
    pub fn response_format(&mut self, response_format: ImageResponseFormat) -> &mut Self {
        self.response_format = response_format.to_string();
        self
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
    /// Default chat arguments
    pub fn default() -> Self {
        Self {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![],
            n: 1,
            temperature: 1.0,
            top_p: 1.0,
            max_tokens: 32,
            presence_penalty: 0.0,
            frequency_penalty: 0.0,
        }
    }

    /// Set the model to use
    pub fn model<T>(&mut self, model: T) -> &mut Self
    where
        T: ToString,
    {
        self.model = model.to_string();
        self
    }

    /// Set the messages to use
    pub fn messages(&mut self, messages: Vec<HashMap<String, String>>) -> &mut Self {
        self.messages = messages;
        self
    }

    /// Set the number of messages to return
    pub fn n(&mut self, n: i32) -> &mut Self {
        self.n = n;
        self
    }

    /// Set the temperature to use
    pub fn temperature(&mut self, temperature: f64) -> &mut Self {
        self.temperature = temperature;
        self
    }

    /// Set the top_p to use
    pub fn top_p(&mut self, top_p: f64) -> &mut Self {
        self.top_p = top_p;
        self
    }

    /// Set the max_tokens to use
    pub fn max_tokens(&mut self, max_tokens: u32) -> &mut Self {
        self.max_tokens = max_tokens;
        self
    }

    /// Set the presence_penalty to use
    pub fn presence_penalty(&mut self, presence_penalty: f64) -> &mut Self {
        self.presence_penalty = presence_penalty;
        self
    }

    /// Set the frequency_penalty to use
    pub fn frequency_penalty(&mut self, frequency_penalty: f64) -> &mut Self {
        self.frequency_penalty = frequency_penalty;
        self
    }
}
