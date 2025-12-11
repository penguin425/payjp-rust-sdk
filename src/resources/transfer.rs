//! Transfer resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::ListParams;
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// A transfer represents a payout to your bank account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    /// Unique identifier for the transfer (prefixed with `tr_`).
    pub id: String,

    /// Object type (always "transfer").
    pub object: String,

    /// Whether this transfer was created in live mode.
    pub livemode: bool,

    /// Transfer creation timestamp (Unix timestamp).
    pub created: i64,

    /// Amount transferred (in smallest currency unit).
    pub amount: i64,

    /// Three-letter ISO currency code.
    pub currency: String,

    /// Transfer status ("pending", "paid", "failed", "stop", or "carried_forward").
    pub status: String,

    /// Summary of charges included in this transfer.
    pub summary: TransferSummary,

    /// Scheduled transfer date (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_date: Option<i64>,

    /// Bank information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<BankInfo>,

    /// Statement descriptor (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Term ID (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
}

/// Summary of charges in a transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSummary {
    /// Total charge amount.
    pub charge_amount: i64,

    /// Total charge count.
    pub charge_count: i64,

    /// Total charge fee.
    pub charge_fee: i64,

    /// Total refund amount.
    pub refund_amount: i64,

    /// Total refund count.
    pub refund_count: i64,
}

/// Bank account information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankInfo {
    /// Bank code.
    pub bank_code: String,

    /// Branch code.
    pub branch_code: String,

    /// Account type ("普通" or "当座").
    pub account_type: String,

    /// Account number.
    pub account_number: String,

    /// Account holder name.
    pub account_holder_name: String,
}

/// Service for retrieving transfers.
pub struct TransferService<'a> {
    client: &'a PayjpClient,
}

impl<'a> TransferService<'a> {
    /// Create a new transfer service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Retrieve a transfer by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let transfer = client.transfers().retrieve("tr_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, transfer_id: &str) -> PayjpResult<Transfer> {
        let path = format!("/transfers/{}", transfer_id);
        self.client.get(&path).await
    }

    /// List all transfers.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let transfers = client.transfers().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<Transfer>> {
        self.client.get_with_params("/transfers", &params).await
    }
}
