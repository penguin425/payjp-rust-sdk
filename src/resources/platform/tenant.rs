//! Tenant resource and service implementation (Platform API).

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::{ListParams, Metadata};
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// A tenant represents a sub-merchant in the platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    /// Unique identifier for the tenant (prefixed with `ten_`).
    pub id: String,

    /// Object type (always "tenant").
    pub object: String,

    /// Whether this tenant was created in live mode.
    pub livemode: bool,

    /// Tenant creation timestamp (Unix timestamp).
    pub created: i64,

    /// Tenant name (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Platform fee rate for this tenant (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_fee_rate: Option<String>,

    /// Minimum transfer amount (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_transfer_amount: Option<i64>,

    /// Bank information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,

    /// Currencies enabled for this tenant (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies_supported: Option<Vec<String>>,

    /// Default currency for this tenant (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<String>,

    /// Set of key-value pairs for storing additional information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Bank account information for a tenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
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

/// Parameters for creating a tenant.
#[derive(Debug, Default, Clone, Serialize)]
pub struct CreateTenantParams {
    /// Tenant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Platform fee rate (as a decimal string, e.g., "0.10" for 10%).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_fee_rate: Option<String>,

    /// Minimum transfer amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_transfer_amount: Option<i64>,

    /// Bank account information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreateTenantParams {
    /// Create new tenant parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the tenant name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the platform fee rate.
    pub fn platform_fee_rate(mut self, rate: impl Into<String>) -> Self {
        self.platform_fee_rate = Some(rate.into());
        self
    }

    /// Set the minimum transfer amount.
    pub fn minimum_transfer_amount(mut self, amount: i64) -> Self {
        self.minimum_transfer_amount = Some(amount);
        self
    }

    /// Set the bank account.
    pub fn bank_account(mut self, account: BankAccount) -> Self {
        self.bank_account = Some(account);
        self
    }

    /// Add metadata to the tenant.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }
}

/// Parameters for updating a tenant.
#[derive(Debug, Default, Clone, Serialize)]
pub struct UpdateTenantParams {
    /// Tenant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Platform fee rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_fee_rate: Option<String>,

    /// Minimum transfer amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_transfer_amount: Option<i64>,

    /// Bank account information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl UpdateTenantParams {
    /// Create new update tenant parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the tenant name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the platform fee rate.
    pub fn platform_fee_rate(mut self, rate: impl Into<String>) -> Self {
        self.platform_fee_rate = Some(rate.into());
        self
    }

    /// Add metadata to the tenant.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }
}

/// Response from deleting a tenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedTenant {
    /// Tenant ID.
    pub id: String,

    /// Whether the deletion was successful.
    pub deleted: bool,

    /// Whether this tenant was in live mode.
    pub livemode: bool,
}

/// Application URLs for tenant onboarding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationUrls {
    /// URL for the application (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Expiration timestamp for the URL (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
}

/// Service for managing tenants (Platform API).
pub struct TenantService<'a> {
    client: &'a PayjpClient,
}

impl<'a> TenantService<'a> {
    /// Create a new tenant service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Create a new tenant.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, CreateTenantParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let tenant = client.tenants().create(
    ///     CreateTenantParams::new()
    ///         .name("Sub-merchant")
    ///         .platform_fee_rate("0.10")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, params: CreateTenantParams) -> PayjpResult<Tenant> {
        self.client.post("/tenants", &params).await
    }

    /// Retrieve a tenant by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let tenant = client.tenants().retrieve("ten_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, tenant_id: &str) -> PayjpResult<Tenant> {
        let path = format!("/tenants/{}", tenant_id);
        self.client.get(&path).await
    }

    /// Update a tenant.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, UpdateTenantParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let tenant = client.tenants().update(
    ///     "ten_xxxxx",
    ///     UpdateTenantParams::new().name("Updated Name")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, tenant_id: &str, params: UpdateTenantParams) -> PayjpResult<Tenant> {
        let path = format!("/tenants/{}", tenant_id);
        self.client.post(&path, &params).await
    }

    /// Delete a tenant.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let deleted = client.tenants().delete("ten_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, tenant_id: &str) -> PayjpResult<DeletedTenant> {
        let path = format!("/tenants/{}", tenant_id);
        self.client.delete(&path).await
    }

    /// List all tenants.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let tenants = client.tenants().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<Tenant>> {
        self.client.get_with_params("/tenants", &params).await
    }

    /// Create application URLs for tenant onboarding.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let urls = client.tenants().create_application_urls("ten_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_application_urls(&self, tenant_id: &str) -> PayjpResult<ApplicationUrls> {
        let path = format!("/tenants/{}/application_urls", tenant_id);
        self.client.post(&path, &serde_json::json!({})).await
    }
}
