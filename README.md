# devto-api

A Rust SDK for the [dev.to](https://dev.to) (Forem) API.

This crate provides a type-safe, async client for interacting with the dev.to API. It is auto-generated from the [Forem OpenAPI specification](https://developers.forem.com/api) using [Progenitor](https://github.com/oxidecomputer/progenitor).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
devto-api = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust
use devto_api::{Client, ClientExt};
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your API key
    let client = Client::devto("your-api-key")?;

    // Fetch latest articles
    let articles = client
        .get_articles(None, None, NonZeroU32::new(10), None, None, None, None, None, None)
        .await?;

    for article in articles.into_inner() {
        println!("{}", article.title);
    }

    Ok(())
}
```

## Usage

### Authentication

Get your API key from [dev.to settings](https://dev.to/settings/extensions).

```rust
use devto_api::{Client, ClientExt};

// Authenticated client (required for creating/updating content)
let client = Client::devto("your-api-key")?;

// Public client (read-only access to public endpoints)
let client = Client::devto_public()?;
```

### Custom Forem Instances

This SDK works with any Forem-powered community:

```rust
use devto_api::{Client, ClientExt};

let client = Client::forem("https://community.example.com/api", "your-api-key")?;
```

### Examples

Fetch articles by tag:

```rust
let rust_articles = client
    .get_articles(None, None, None, None, Some("rust"), None, None, None, None)
    .await?;
```

Get a specific article:

```rust
let article = client.get_article_by_id(12345).await?;
println!("Title: {}", article.into_inner().title);
```

See the [examples](./examples) directory for more usage examples.

## Features

- Type-safe API bindings generated from OpenAPI spec
- Async/await support with Tokio
- Support for authenticated and public endpoints
- Works with dev.to and any Forem-powered community

## API Coverage

This SDK covers the [Forem API v1](https://developers.forem.com/api), including:

- Articles (list, create, update, publish)
- Comments
- Users and profiles
- Organizations
- Tags
- Podcasts
- Videos
- Listings
- Followers/Following
- Reactions
- And more

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
