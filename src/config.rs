//! Client configuration helpers for dev.to API.

use crate::Client;

/// The default base URL for the dev.to API.
pub const DEVTO_API_URL: &str = "https://dev.to/api";

/// Extension trait providing convenient constructors for the dev.to API client.
pub trait ClientExt {
    /// Create a new client configured for the dev.to API with an API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your dev.to API key. You can generate one at https://dev.to/settings/extensions
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use devto_api::{Client, ClientExt};
    ///
    /// let client = Client::devto("your-api-key").unwrap();
    /// ```
    fn devto(api_key: &str) -> Result<Client, DevtoClientError>;

    /// Create a new client configured for the dev.to API without authentication.
    ///
    /// This client can only access public endpoints.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use devto_api::{Client, ClientExt};
    ///
    /// let client = Client::devto_public().unwrap();
    /// ```
    fn devto_public() -> Result<Client, DevtoClientError>;

    /// Create a new client for a custom Forem instance with an API key.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Forem instance (e.g., "https://community.example.com/api")
    /// * `api_key` - Your API key for the Forem instance
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use devto_api::{Client, ClientExt};
    ///
    /// let client = Client::forem("https://community.example.com/api", "your-api-key").unwrap();
    /// ```
    fn forem(base_url: &str, api_key: &str) -> Result<Client, DevtoClientError>;
}

/// Errors that can occur when creating a dev.to client.
#[derive(Debug, thiserror::Error)]
pub enum DevtoClientError {
    /// Failed to build the HTTP client.
    #[error("failed to build HTTP client: {0}")]
    HttpClient(#[from] reqwest::Error),

    /// Invalid base URL.
    #[error("invalid base URL: {0}")]
    InvalidUrl(String),
}

impl ClientExt for Client {
    fn devto(api_key: &str) -> Result<Client, DevtoClientError> {
        Self::forem(DEVTO_API_URL, api_key)
    }

    fn devto_public() -> Result<Client, DevtoClientError> {
        let http_client = reqwest::Client::builder()
            .user_agent(concat!("devto-api-rust/", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Client::new_with_client(DEVTO_API_URL, http_client))
    }

    fn forem(base_url: &str, api_key: &str) -> Result<Client, DevtoClientError> {
        use reqwest::header::{HeaderMap, HeaderValue};

        let mut headers = HeaderMap::new();
        headers.insert(
            "api-key",
            HeaderValue::from_str(api_key)
                .map_err(|_| DevtoClientError::InvalidUrl("Invalid API key format".into()))?,
        );

        let http_client = reqwest::Client::builder()
            .user_agent(concat!("devto-api-rust/", env!("CARGO_PKG_VERSION")))
            .default_headers(headers)
            .build()?;

        Ok(Client::new_with_client(base_url, http_client))
    }
}
