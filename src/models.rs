use std::fmt::Display;

#[derive(Debug)]
pub enum CompletionModels {
    TextDavinci3,
    TextDavinci2,
    TextCurie1,
    TextBabbage1,
    TextAda1,
    CodeDavinci2,
}

impl Display for CompletionModels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompletionModels::TextDavinci3 => write!(f, "text-davinci-003"),
            CompletionModels::TextDavinci2 => write!(f, "text-davinci-002"),
            CompletionModels::TextCurie1 => write!(f, "text-curie-001"),
            CompletionModels::TextBabbage1 => write!(f, "text-babbage-001"),
            CompletionModels::TextAda1 => write!(f, "text-ada-001"),
            CompletionModels::CodeDavinci2 => write!(f, "code-davinci-002"),
        }
    }
}

pub enum EditModels {
    TextDavinciEdit1,
    CodeDavinciEdit1,
}

impl Display for EditModels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditModels::TextDavinciEdit1 => write!(f, "text-davinci-edit-001"),
            EditModels::CodeDavinciEdit1 => write!(f, "code-davinci-edit-001"),
        }
    }
}

pub enum ChatModels {
    Gpt4,
    Gpt4_0314,
    Gpt4_32k,
    Gpt4_32k0314,
    Gpt3_5Turbo,
    Gpt3_5Turbo0301,
}

impl Display for ChatModels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatModels::Gpt4 => write!(f, "gpt-4"),
            ChatModels::Gpt4_0314 => write!(f, "gpt-4-0314"),
            ChatModels::Gpt4_32k => write!(f, "gpt-4-32k"),
            ChatModels::Gpt4_32k0314 => write!(f, "gpt-4-32k-0314"),
            ChatModels::Gpt3_5Turbo => write!(f, "gpt-3-5-turbo"),
            ChatModels::Gpt3_5Turbo0301 => write!(f, "gpt-3-5-turbo-0301"),
        }
    }
}
