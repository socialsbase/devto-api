//! Basic usage example for the dev.to API client.
//!
//! Run with: cargo run --example basic_usage
//!
//! Set the VIBE_FOREM_API_KEY environment variable to use authenticated endpoints.

use devto_api::{Client, ClientExt};
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client - use API key if available, otherwise public access
    let client = match std::env::var("VIBE_FOREM_API_KEY") {
        Ok(api_key) => {
            println!("Using authenticated client");
            Client::devto(&api_key)?
        }
        Err(_) => {
            println!("Using public client (set VIBE_FOREM_API_KEY for authenticated access)");
            Client::devto_public()?
        }
    };

    // Fetch latest articles (5 per page)
    println!("\n--- Latest Articles ---");
    let per_page = NonZeroU32::new(5);
    let articles = client
        .get_articles(
            None,       // collection_id
            None,       // page
            per_page,   // per_page
            None,       // state
            None,       // tag
            None,       // tags
            None,       // tags_exclude
            None,       // top
            None,       // username
        )
        .await?;

    for article in articles.into_inner() {
        let author = article.user.name.as_deref().unwrap_or("Unknown");
        println!("- {} by {} (ID: {})", article.title, author, article.id);
    }

    // Fetch articles with a specific tag
    println!("\n--- Rust Articles ---");
    let rust_articles = client
        .get_articles(
            None,                // collection_id
            None,                // page
            per_page,            // per_page
            None,                // state
            Some("rust"),        // tag
            None,                // tags
            None,                // tags_exclude
            None,                // top
            None,                // username
        )
        .await?;

    for article in rust_articles.into_inner() {
        println!("- {}", article.title);
    }

    // Fetch a specific article by ID
    println!("\n--- Article Details ---");
    let article = client.get_article_by_id(1).await?;
    let article = article.into_inner();

    println!("Title: {}", article.title);
    println!("URL: {}", article.url);
    println!("Description: {}", article.description);
    println!("Reading time: {} minutes", article.reading_time_minutes);

    println!("\nDone!");
    Ok(())
}
