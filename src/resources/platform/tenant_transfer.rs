//! Tenant transfer resource and service implementation (Platform API).

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::ListParams;
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// A tenant transfer represents a payout to a tenant's bank account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantTransfer {
    /// Unique identifier for the tenant transfer (prefixed with `ttr_`).
    pub id: String,

    /// Object type (always "tenant_transfer").
    pub object: String,

    /// Whether this transfer was created in live mode.
    pub livemode: bool,

    /// Transfer creation timestamp (Unix timestamp).
    pub created: i64,

    /// Tenant ID.
    pub tenant: String,

    /// Amount transferred (in smallest currency unit).
    pub amount: i64,

    /// Three-letter ISO currency code.
    pub currency: String,

    /// Transfer status.
    pub status: String,

    /// Summary of charges included in this transfer.
    pub summary: TenantTransferSummary,

    /// Scheduled transfer date (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_date: Option<i64>,

    /// Term ID (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
}

/// Summary of charges in a tenant transfer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantTransferSummary {
    /// Total charge amount.
    pub charge_amount: i64,

    /// Total charge count.
    pub charge_count: i64,

    /// Total charge fee.
    pub charge_fee: i64,

    /// Total platform fee.
    pub platform_fee: i64,

    /// Total refund amount.
    pub refund_amount: i64,

    /// Total refund count.
    pub refund_count: i64,
}

/// Service for retrieving tenant transfers (Platform API).
pub struct TenantTransferService<'a> {
    client: &'a PayjpClient,
}

impl<'a> TenantTransferService<'a> {
    /// Create a new tenant transfer service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Retrieve a tenant transfer by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let transfer = client.tenant_transfers().retrieve("ttr_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, transfer_id: &str) -> PayjpResult<TenantTransfer> {
        let path = format!("/tenant_transfers/{}", transfer_id);
        self.client.get(&path).await
    }

    /// List all tenant transfers.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let transfers = client.tenant_transfers().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<TenantTransfer>> {
        self.client.get_with_params("/tenant_transfers", &params).await
    }
}
