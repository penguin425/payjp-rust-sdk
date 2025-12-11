//! Statement resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::ListParams;
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// A statement represents a transaction details report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statement {
    /// Unique identifier for the statement (prefixed with `st_`).
    pub id: String,

    /// Object type (always "statement").
    pub object: String,

    /// Whether this statement was created in live mode.
    pub livemode: bool,

    /// Statement creation timestamp (Unix timestamp).
    pub created: i64,

    /// Title of the statement (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Tenant ID (Platform API, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,

    /// Term ID (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,

    /// Balance ID (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_id: Option<String>,

    /// Statement type (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_type: Option<String>,

    /// Updated timestamp (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<i64>,
}

/// Statement URLs response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementUrls {
    /// Object type (always "statement_urls").
    pub object: String,

    /// Expiration timestamp for the URLs (Unix timestamp).
    pub expires: i64,

    /// URL for the statement (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Service for retrieving statements.
pub struct StatementService<'a> {
    client: &'a PayjpClient,
}

impl<'a> StatementService<'a> {
    /// Create a new statement service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Retrieve a statement by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let statement = client.statements().retrieve("st_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, statement_id: &str) -> PayjpResult<Statement> {
        let path = format!("/statements/{}", statement_id);
        self.client.get(&path).await
    }

    /// Get download URLs for a statement.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let urls = client.statements().statement_urls("st_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn statement_urls(&self, statement_id: &str) -> PayjpResult<StatementUrls> {
        let path = format!("/statements/{}/statement_urls", statement_id);
        self.client.post(&path, &serde_json::json!({})).await
    }

    /// List all statements.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let statements = client.statements().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<Statement>> {
        self.client.get_with_params("/statements", &params).await
    }
}
