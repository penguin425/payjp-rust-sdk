//! PAY.JP API client implementation.

use crate::error::{ErrorResponse, PayjpError, PayjpResult};
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

/// Default base URL for PAY.JP API.
pub const DEFAULT_BASE_URL: &str = "https://api.pay.jp/v1";

/// Default maximum number of retry attempts.
pub const DEFAULT_MAX_RETRY: u32 = 3;

/// Default initial retry delay (500ms).
pub const DEFAULT_RETRY_INITIAL_DELAY: Duration = Duration::from_millis(500);

/// Default maximum retry delay (10 seconds).
pub const DEFAULT_RETRY_MAX_DELAY: Duration = Duration::from_secs(10);

/// Configuration options for the PAY.JP client.
#[derive(Debug, Clone)]
pub struct ClientOptions {
    /// Base URL for the API (default: https://api.pay.jp/v1).
    pub base_url: String,

    /// Maximum number of retry attempts for rate-limited requests.
    pub max_retry: u32,

    /// Initial delay before the first retry.
    pub retry_initial_delay: Duration,

    /// Maximum delay between retries.
    pub retry_max_delay: Duration,

    /// HTTP client timeout.
    pub timeout: Duration,
}

impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            max_retry: DEFAULT_MAX_RETRY,
            retry_initial_delay: DEFAULT_RETRY_INITIAL_DELAY,
            retry_max_delay: DEFAULT_RETRY_MAX_DELAY,
            timeout: Duration::from_secs(30),
        }
    }
}

impl ClientOptions {
    /// Create a new `ClientOptions` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL for the API.
    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_string();
        self
    }

    /// Set the maximum number of retry attempts.
    pub fn max_retry(mut self, max_retry: u32) -> Self {
        self.max_retry = max_retry;
        self
    }

    /// Set the initial retry delay.
    pub fn retry_initial_delay(mut self, delay: Duration) -> Self {
        self.retry_initial_delay = delay;
        self
    }

    /// Set the maximum retry delay.
    pub fn retry_max_delay(mut self, delay: Duration) -> Self {
        self.retry_max_delay = delay;
        self
    }

    /// Set the HTTP client timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

/// The main PAY.JP API client.
#[derive(Debug, Clone)]
pub struct PayjpClient {
    api_key: String,
    http_client: reqwest::Client,
    base_url: String,
    max_retry: u32,
    retry_initial_delay: Duration,
    retry_max_delay: Duration,
}

impl PayjpClient {
    /// Create a new PAY.JP client with the given API key.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payjp::PayjpClient;
    ///
    /// let client = PayjpClient::new("sk_test_xxxxx");
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_options(api_key, ClientOptions::default())
    }

    /// Create a new PAY.JP client with custom options.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use payjp::{PayjpClient, ClientOptions};
    /// use std::time::Duration;
    ///
    /// let options = ClientOptions::new()
    ///     .timeout(Duration::from_secs(60))
    ///     .max_retry(5);
    ///
    /// let client = PayjpClient::with_options("sk_test_xxxxx", options);
    /// ```
    pub fn with_options(api_key: impl Into<String>, options: ClientOptions) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(options.timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key: api_key.into(),
            http_client,
            base_url: options.base_url,
            max_retry: options.max_retry,
            retry_initial_delay: options.retry_initial_delay,
            retry_max_delay: options.retry_max_delay,
        }
    }

    /// Get the base URL for the API.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Send a GET request.
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> PayjpResult<T> {
        self.request_with_retry(Method::GET, path, None::<&()>)
            .await
    }

    /// Send a GET request with query parameters.
    pub(crate) async fn get_with_params<T: DeserializeOwned, P: Serialize>(
        &self,
        path: &str,
        params: &P,
    ) -> PayjpResult<T> {
        self.request_with_retry(Method::GET, path, Some(params))
            .await
    }

    /// Send a POST request.
    pub(crate) async fn post<T: DeserializeOwned, P: Serialize>(
        &self,
        path: &str,
        params: &P,
    ) -> PayjpResult<T> {
        self.request_with_retry(Method::POST, path, Some(params))
            .await
    }

    /// Send a DELETE request.
    pub(crate) async fn delete<T: DeserializeOwned>(&self, path: &str) -> PayjpResult<T> {
        self.request_with_retry(Method::DELETE, path, None::<&()>)
            .await
    }

    /// Send a request with retry logic for rate limiting.
    async fn request_with_retry<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&impl Serialize>,
    ) -> PayjpResult<T> {
        let mut retry_count = 0;

        loop {
            match self.send_request(method.clone(), path, body).await {
                Ok(response) => return Ok(response),
                Err(PayjpError::RateLimit) if retry_count < self.max_retry => {
                    let delay = self.calculate_retry_delay(retry_count);
                    tokio::time::sleep(delay).await;
                    retry_count += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Calculate retry delay with exponential backoff and jitter.
    fn calculate_retry_delay(&self, retry_count: u32) -> Duration {
        let base = self.retry_initial_delay.as_millis() as u64
            .saturating_mul(2u64.saturating_pow(retry_count));
        let max = self.retry_max_delay.as_millis() as u64;
        let capped = base.min(max);

        // Equal jitter: random between capped/2 and capped
        let jittered = capped / 2 + rand::rng().random_range(0..=capped / 2);
        Duration::from_millis(jittered)
    }

    /// Send an HTTP request to the PAY.JP API.
    async fn send_request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&impl Serialize>,
    ) -> PayjpResult<T> {
        let url = format!("{}{}", self.base_url, path);

        // Create basic auth header
        let auth = format!("{}:", self.api_key);
        let encoded = general_purpose::STANDARD.encode(auth.as_bytes());
        let auth_header = format!("Basic {}", encoded);

        let mut request = self
            .http_client
            .request(method.clone(), &url)
            .header("Authorization", auth_header)
            .header("User-Agent", "payjp-rust/0.1.0");

        // Add body based on method
        request = if method == Method::GET {
            if let Some(params) = body {
                request.query(params)
            } else {
                request
            }
        } else if let Some(params) = body {
            request.form(params)
        } else {
            request
        };

        let response = request.send().await?;
        let status = response.status();

        // Handle different status codes
        match status {
            StatusCode::OK | StatusCode::CREATED => {
                let data = response.json::<T>().await?;
                Ok(data)
            }
            StatusCode::TOO_MANY_REQUESTS => Err(PayjpError::RateLimit),
            StatusCode::UNAUTHORIZED => {
                Err(PayjpError::Auth("Invalid API key".to_string()))
            }
            _ => {
                // Try to parse error response
                if let Ok(error_response) = response.json::<ErrorResponse>().await {
                    Err(PayjpError::Api(error_response.error))
                } else {
                    Err(PayjpError::Api(crate::error::ApiError {
                        status: status.as_u16(),
                        error_type: "unknown_error".to_string(),
                        message: format!("HTTP error: {}", status),
                        code: None,
                        param: None,
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = PayjpClient::new("sk_test_xxxxx");
        assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    }

    #[test]
    fn test_client_with_options() {
        let options = ClientOptions::new()
            .base_url("https://custom.api.pay.jp/v1")
            .max_retry(5);

        let client = PayjpClient::with_options("sk_test_xxxxx", options);
        assert_eq!(client.base_url(), "https://custom.api.pay.jp/v1");
        assert_eq!(client.max_retry, 5);
    }

    #[test]
    fn test_retry_delay_calculation() {
        let client = PayjpClient::new("sk_test_xxxxx");

        // Test that delay is within expected range
        for retry_count in 0..5 {
            let delay = client.calculate_retry_delay(retry_count);
            let expected_base = DEFAULT_RETRY_INITIAL_DELAY.as_millis() as u64
                * 2u64.pow(retry_count);
            let expected_max = expected_base.min(DEFAULT_RETRY_MAX_DELAY.as_millis() as u64);

            assert!(delay.as_millis() as u64 >= expected_max / 2);
            assert!(delay.as_millis() as u64 <= expected_max);
        }
    }
}
