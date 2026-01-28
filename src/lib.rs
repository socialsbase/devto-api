//! # devto-api
//!
//! Auto-generated Rust SDK for the dev.to (Forem) API.
//!
//! This crate provides a type-safe, async client for interacting with the dev.to API.
//! It is generated from the Forem OpenAPI specification using Progenitor.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use devto_api::{Client, ClientExt};
//! use std::num::NonZeroU32;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with your API key
//!     let client = Client::devto("your-api-key")?;
//!
//!     // Fetch latest articles
//!     let articles = client.get_articles(
//!         None, None, NonZeroU32::new(10), None, None, None, None, None, None
//!     ).await?;
//!
//!     for article in articles.into_inner() {
//!         println!("{}", article.title);
//!     }
//!
//!     Ok(())
//! }
//! ```

mod config;

// Include the generated code from build.rs
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub use config::ClientExt;
