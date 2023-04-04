# openai.rs

A wrapper for OpenAI API written in Rust

[![Rust](https://github.com/obvMellow/openai.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/obvMellow/openai.rs/actions/workflows/rust.yml)
[![rust-clippy analyze](https://github.com/obvMellow/openai.rs/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/obvMellow/openai.rs/actions/workflows/rust-clippy.yml)
[![Crates.io](https://img.shields.io/crates/v/openai_gpt_rs)](https://crates.io/crates/openai_gpt_rs)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/c4d2a6bb48d74561a717cdbb8e6e85b6)](https://www.codacy.com/gh/obvMellow/openai.rs/dashboard?utm_source=github.com&utm_medium=referral&utm_content=obvMellow/openai.rs&utm_campaign=Badge_Grade)

## Usage Example

First of all, you need to initialize a client.

```rust ignore
use openai_gpt_rs::client::Client;

let client = Client::new("Your API key goes here");
```

Then you can use the methods to interact with the API:

```rust ignore
let completion = client.create_completion(|args| {
    args.prompt("Say this is a test")
        .n(2)
})
    .await
    .unwrap();
```

This method returns the response returned from the API.

You can do the following to get the content from the response:

```rust ignore
use openai_rs::response::Content;

// Get the text data from the response
let text = completion.get_content(0).unwrap(); // Get the first content

let text_by_range = completion.get_contents(0..2); // Get a vector of strings by a range

// Print the completion
for text in text_by_range {
    println!("{}", text);
}
```

Like in this example, you can get the content by index or by a range.

If you want to get the json, you can directly get access the `json` field:

```rust ignore
let response = completion.json;
```

To see examples of other APIs [you can look at the examples here.](https://github.com/obvMellow/openai.rs/tree/main/examples)

## Features to come

- ~~Chat API support~~
- Embeddings API support
- Audio API support
- Fine tunes API support
