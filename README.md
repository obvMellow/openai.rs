# openai.rs
A wrapper for OpenAI API written in Rust

[![Rust](https://github.com/obvMellow/openai.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/obvMellow/openai.rs/actions/workflows/rust.yml)
[![CodeFactor](https://www.codefactor.io/repository/github/obvmellow/openai.rs/badge)](https://www.codefactor.io/repository/github/obvmellow/openai.rs)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/c4d2a6bb48d74561a717cdbb8e6e85b6)](https://www.codacy.com/gh/obvMellow/openai.rs/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=obvMellow/openai.rs&amp;utm_campaign=Badge_Grade)

## Usage
First of all, you need to initialize a client.

```rust
let client = Client::new("Your API key goes here");
```

Then you can use the methods to interact with the API:

```rust
let args = CompletionArgs::new("Say this is a test",
    Some(32), // Max tokens
    Some(2), // Amount of completions (default: 1)
    None, // Suffix (default: "")
    Some(1.0) // Temperature
);

let completion = client.create_completion(&args)
    .await
    .unwrap();
```
This method returns the response returned from the API.

You can do the following to get the content from the response:

```rust
// Get the text data from the response
let text = completion.get_content(0)
    .await
    .unwrap();
    
// Print the completion
println!("Completion: {}", text);
```
In this example we use `.unwrap()` because `get_content` method returns an `Option<String>`.


If you want to get the actual `Response` you can directly get access the `resp` field:

```rust
// Moves the Response value to the new variable
let response = completion.resp;
```
```rust
// Borrows the Response
let response = &completion.resp;
```

To see examples of other APIs [you can look at the examples here.](https://github.com/obvMellow/openai.rs/tree/main/examples)

## Features to come
-   Chat API support
-   Embeddings API support
-   Audio API support
-   Fine tunes API support
