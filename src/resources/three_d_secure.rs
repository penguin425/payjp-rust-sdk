//! Three-D Secure request resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::ListParams;
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// A 3D Secure request for card authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeDSecureRequest {
    /// Unique identifier for the 3DS request (prefixed with `tdsr_`).
    pub id: String,

    /// Object type (always "three_d_secure_request").
    pub object: String,

    /// Whether this request was created in live mode.
    pub livemode: bool,

    /// Request creation timestamp (Unix timestamp).
    pub created: i64,

    /// Resource type being authenticated ("token" or "charge").
    pub resource_type: String,

    /// Resource ID (token or charge ID, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,

    /// 3DS authentication status.
    pub status: ThreeDSecureStatus,

    /// URL for 3DS authentication (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_url: Option<String>,

    /// Tenant ID (Platform API, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,

    /// State parameter for callback (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Result information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ThreeDSecureResult>,
}

/// Status of a 3D Secure request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ThreeDSecureStatus {
    /// Verification in progress.
    InProgress,

    /// Verification succeeded.
    Verified,

    /// Verification attempted but not completed.
    Attempted,

    /// Verification failed.
    Failed,

    /// Verification error occurred.
    Error,

    /// Verification was aborted.
    Aborted,
}

/// Result of a 3D Secure authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeDSecureResult {
    /// Result code (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Result message (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// ECI (Electronic Commerce Indicator) value (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eci: Option<String>,
}

/// Parameters for creating a 3D Secure request.
#[derive(Debug, Clone, Serialize)]
pub struct CreateThreeDSecureRequestParams {
    /// Resource type ("token" or "charge").
    pub resource_type: String,

    /// Resource ID (token or charge ID).
    pub resource_id: String,

    /// Return URL after authentication (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// State parameter to pass through callback (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Tenant ID (Platform API, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

impl CreateThreeDSecureRequestParams {
    /// Create new 3DS request parameters.
    pub fn new(resource_type: impl Into<String>, resource_id: impl Into<String>) -> Self {
        Self {
            resource_type: resource_type.into(),
            resource_id: resource_id.into(),
            return_url: None,
            state: None,
            tenant: None,
        }
    }

    /// Set the return URL.
    pub fn return_url(mut self, url: impl Into<String>) -> Self {
        self.return_url = Some(url.into());
        self
    }

    /// Set the state parameter.
    pub fn state(mut self, state: impl Into<String>) -> Self {
        self.state = Some(state.into());
        self
    }

    /// Set the tenant ID (Platform API).
    pub fn tenant(mut self, tenant: impl Into<String>) -> Self {
        self.tenant = Some(tenant.into());
        self
    }
}

/// Service for managing 3D Secure requests.
pub struct ThreeDSecureRequestService<'a> {
    client: &'a PayjpClient,
}

impl<'a> ThreeDSecureRequestService<'a> {
    /// Create a new 3D Secure request service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Create a new 3D Secure request.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, CreateThreeDSecureRequestParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let tds_request = client.three_d_secure_requests().create(
    ///     CreateThreeDSecureRequestParams::new("token", "tok_xxxxx")
    ///         .return_url("https://example.com/callback")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        params: CreateThreeDSecureRequestParams,
    ) -> PayjpResult<ThreeDSecureRequest> {
        self.client.post("/three_d_secure_requests", &params).await
    }

    /// Retrieve a 3D Secure request by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let tds_request = client.three_d_secure_requests().retrieve("tdsr_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, request_id: &str) -> PayjpResult<ThreeDSecureRequest> {
        let path = format!("/three_d_secure_requests/{}", request_id);
        self.client.get(&path).await
    }

    /// List all 3D Secure requests.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let tds_requests = client.three_d_secure_requests().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<ThreeDSecureRequest>> {
        self.client
            .get_with_params("/three_d_secure_requests", &params)
            .await
    }
}
