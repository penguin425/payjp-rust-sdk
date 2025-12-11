//! Account resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::Metadata;
use serde::{Deserialize, Serialize};

/// Account information for the authenticated merchant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Unique identifier for the account (prefixed with `acct_`).
    pub id: String,

    /// Object type (always "account").
    pub object: String,

    /// Whether this account is in live mode.
    pub livemode: bool,

    /// Account creation timestamp (Unix timestamp).
    pub created: i64,

    /// Merchant email address (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Merchant name (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,

    /// Business type (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<String>,

    /// Currencies enabled for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies_supported: Option<Vec<String>>,

    /// Default currency for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<String>,

    /// Product detail information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_detail: Option<String>,

    /// Set of key-value pairs for storing additional information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Service for retrieving account information.
pub struct AccountService<'a> {
    client: &'a PayjpClient,
}

impl<'a> AccountService<'a> {
    /// Create a new account service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Retrieve the account information for the authenticated merchant.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let account = client.account().retrieve().await?;
    /// println!("Merchant: {:?}", account.merchant_name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self) -> PayjpResult<Account> {
        self.client.get("/account").await
    }
}
