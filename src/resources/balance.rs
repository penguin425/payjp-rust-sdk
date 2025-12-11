//! Balance resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::ListParams;
use crate::resources::statement::StatementUrls;
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// A balance represents the account balance state at a specific point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// Unique identifier for the balance (prefixed with `ba_`).
    pub id: String,

    /// Object type (always "balance").
    pub object: String,

    /// Whether this balance was created in live mode.
    pub livemode: bool,

    /// Balance creation timestamp (Unix timestamp).
    pub created: i64,

    /// Total balance amount.
    pub total: i64,

    /// Available balance amount.
    pub available: i64,

    /// Pending balance amount.
    pub pending: i64,

    /// Balance state ("processing", "confirmed", etc., optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Tenant ID (Platform API, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,

    /// Bank information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_info: Option<BankInfo>,

    /// Closed at timestamp (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<i64>,

    /// Due date timestamp (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<i64>,
}

/// Bank account information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankInfo {
    /// Bank code.
    pub bank_code: String,

    /// Branch code.
    pub branch_code: String,

    /// Account type.
    pub account_type: String,

    /// Account number.
    pub account_number: String,

    /// Account holder name.
    pub account_holder_name: String,
}

/// Service for retrieving balances.
pub struct BalanceService<'a> {
    client: &'a PayjpClient,
}

impl<'a> BalanceService<'a> {
    /// Create a new balance service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Retrieve a balance by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let balance = client.balances().retrieve("ba_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, balance_id: &str) -> PayjpResult<Balance> {
        let path = format!("/balances/{}", balance_id);
        self.client.get(&path).await
    }

    /// Get download URLs for a balance statement.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let urls = client.balances().statement_urls("ba_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn statement_urls(&self, balance_id: &str) -> PayjpResult<StatementUrls> {
        let path = format!("/balances/{}/statement_urls", balance_id);
        self.client.post(&path, &serde_json::json!({})).await
    }

    /// List all balances.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let balances = client.balances().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<Balance>> {
        self.client.get_with_params("/balances", &params).await
    }
}
