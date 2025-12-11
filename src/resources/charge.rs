//! Charge resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::{ListParams, Metadata};
use crate::resources::card::{Card, CardThreeDSecureStatus};
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// A charge represents a payment against a card or customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Charge {
    /// Unique identifier for the charge (prefixed with `ch_`).
    pub id: String,

    /// Object type (always "charge").
    pub object: String,

    /// Whether this charge was created in live mode.
    pub livemode: bool,

    /// Charge creation timestamp (Unix timestamp).
    pub created: i64,

    /// Amount in the smallest currency unit (e.g., cents for USD, yen for JPY).
    pub amount: i64,

    /// Three-letter ISO currency code (e.g., "jpy").
    pub currency: String,

    /// Whether the charge has been paid.
    pub paid: bool,

    /// Whether the charge has been captured (for auth-only charges).
    pub captured: bool,

    /// Timestamp when the charge was captured (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captured_at: Option<i64>,

    /// Card used for this charge (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,

    /// Customer ID (if charge was made against a customer, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Description of the charge (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Failure code (if charge failed, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,

    /// Failure message (if charge failed, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,

    /// Fee rate applied to this charge (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<String>,

    /// Whether the charge has been refunded.
    pub refunded: bool,

    /// Amount refunded in the smallest currency unit.
    pub amount_refunded: i64,

    /// Reason for refund (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_reason: Option<String>,

    /// Subscription ID (if charge was created by a subscription, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,

    /// Set of key-value pairs for storing additional information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Expiration timestamp for uncaptured charges (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<i64>,

    /// 3D Secure status (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure_status: Option<CardThreeDSecureStatus>,

    /// Platform API: Tenant ID (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,

    /// Platform API: Platform fee amount (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_fee: Option<i64>,

    /// Platform API: Platform fee rate (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_fee_rate: Option<String>,

    /// Platform API: Total platform fee (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_platform_fee: Option<i64>,
}

/// Parameters for creating a charge.
#[derive(Debug, Default, Clone, Serialize)]
pub struct CreateChargeParams {
    /// Amount in the smallest currency unit (JPY: 50-9999999).
    pub amount: i64,

    /// Three-letter ISO currency code (currently only "jpy" is supported).
    pub currency: String,

    /// Card token ID (required if customer is not provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,

    /// Customer ID (required if card is not provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Description of the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether to immediately capture the charge (default: true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,

    /// Number of days before uncaptured charge expires (1-60, default: 7).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_days: Option<i64>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Whether to use 3D Secure authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<bool>,

    /// Platform API: Tenant ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,

    /// Platform API: Platform fee amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_fee: Option<i64>,
}

impl CreateChargeParams {
    /// Create new charge parameters with an amount and currency.
    pub fn new(amount: i64, currency: impl Into<String>) -> Self {
        Self {
            amount,
            currency: currency.into(),
            ..Default::default()
        }
    }

    /// Set the card token to charge.
    pub fn card(mut self, card: impl Into<String>) -> Self {
        self.card = Some(card.into());
        self
    }

    /// Set the customer to charge.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.customer = Some(customer.into());
        self
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set whether to capture immediately.
    pub fn capture(mut self, capture: bool) -> Self {
        self.capture = Some(capture);
        self
    }

    /// Set the number of days before expiration for uncaptured charges.
    pub fn expiry_days(mut self, days: i64) -> Self {
        self.expiry_days = Some(days);
        self
    }

    /// Add metadata to the charge.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }

    /// Enable 3D Secure authentication.
    pub fn three_d_secure(mut self, enabled: bool) -> Self {
        self.three_d_secure = Some(enabled);
        self
    }

    /// Set platform fee (Platform API).
    pub fn platform_fee(mut self, fee: i64) -> Self {
        self.platform_fee = Some(fee);
        self
    }

    /// Set tenant (Platform API).
    pub fn tenant(mut self, tenant: impl Into<String>) -> Self {
        self.tenant = Some(tenant.into());
        self
    }
}

/// Parameters for updating a charge.
#[derive(Debug, Default, Clone, Serialize)]
pub struct UpdateChargeParams {
    /// Description of the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl UpdateChargeParams {
    /// Create new update charge parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add metadata to the charge.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }
}

/// Parameters for refunding a charge.
#[derive(Debug, Default, Clone, Serialize)]
pub struct RefundParams {
    /// Amount to refund (optional, defaults to full charge amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Reason for the refund (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_reason: Option<String>,
}

impl RefundParams {
    /// Create new refund parameters for a full refund.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the amount to refund (for partial refunds).
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set the reason for the refund.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.refund_reason = Some(reason.into());
        self
    }
}

/// Parameters for capturing a charge.
#[derive(Debug, Default, Clone, Serialize)]
pub struct CaptureParams {
    /// Amount to capture (optional, defaults to full authorized amount).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}

impl CaptureParams {
    /// Create new capture parameters for full capture.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the amount to capture (for partial captures).
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }
}

/// Parameters for re-authorizing a charge.
#[derive(Debug, Default, Clone, Serialize)]
pub struct ReauthParams {
    /// Number of days before the new expiration (1-60, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_days: Option<i64>,
}

impl ReauthParams {
    /// Create new reauth parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the number of days before expiration.
    pub fn expiry_days(mut self, days: i64) -> Self {
        self.expiry_days = Some(days);
        self
    }
}

/// Parameters for listing charges.
#[derive(Debug, Default, Clone, Serialize)]
pub struct ListChargeParams {
    /// Maximum number of items to return (default: 10, max: 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Offset for pagination (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Return charges created since this timestamp (Unix timestamp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,

    /// Return charges created until this timestamp (Unix timestamp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,

    /// Filter by customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Filter by subscription ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,

    /// Filter by tenant ID (Platform API).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

impl From<ListParams> for ListChargeParams {
    fn from(params: ListParams) -> Self {
        Self {
            limit: params.limit,
            offset: params.offset,
            since: params.since,
            until: params.until,
            ..Default::default()
        }
    }
}

impl ListChargeParams {
    /// Create new list charge parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit for the number of items to return.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination.
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Filter by customer ID.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.customer = Some(customer.into());
        self
    }

    /// Filter by subscription ID.
    pub fn subscription(mut self, subscription: impl Into<String>) -> Self {
        self.subscription = Some(subscription.into());
        self
    }
}

/// Service for managing charges.
pub struct ChargeService<'a> {
    client: &'a PayjpClient,
}

impl<'a> ChargeService<'a> {
    /// Create a new charge service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Create a new charge.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, CreateChargeParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let charge = client.charges().create(
    ///     CreateChargeParams::new(1000, "jpy")
    ///         .card("tok_xxxxx")
    ///         .description("Test charge")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, params: CreateChargeParams) -> PayjpResult<Charge> {
        self.client.post("/charges", &params).await
    }

    /// Retrieve a charge by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let charge = client.charges().retrieve("ch_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, charge_id: &str) -> PayjpResult<Charge> {
        let path = format!("/charges/{}", charge_id);
        self.client.get(&path).await
    }

    /// Update a charge.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, UpdateChargeParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let charge = client.charges().update(
    ///     "ch_xxxxx",
    ///     UpdateChargeParams::new().description("Updated description")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, charge_id: &str, params: UpdateChargeParams) -> PayjpResult<Charge> {
        let path = format!("/charges/{}", charge_id);
        self.client.post(&path, &params).await
    }

    /// Capture a previously authorized charge.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, CaptureParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let charge = client.charges().capture("ch_xxxxx", CaptureParams::new()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn capture(&self, charge_id: &str, params: CaptureParams) -> PayjpResult<Charge> {
        let path = format!("/charges/{}/capture", charge_id);
        self.client.post(&path, &params).await
    }

    /// Refund a charge.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, RefundParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let charge = client.charges().refund(
    ///     "ch_xxxxx",
    ///     RefundParams::new().reason("Customer request")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn refund(&self, charge_id: &str, params: RefundParams) -> PayjpResult<Charge> {
        let path = format!("/charges/{}/refund", charge_id);
        self.client.post(&path, &params).await
    }

    /// Re-authorize a charge (extend expiration for uncaptured charge).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ReauthParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let charge = client.charges().reauth("ch_xxxxx", ReauthParams::new()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn reauth(&self, charge_id: &str, params: ReauthParams) -> PayjpResult<Charge> {
        let path = format!("/charges/{}/reauth", charge_id);
        self.client.post(&path, &params).await
    }

    /// Finish 3D Secure authentication for a charge.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let charge = client.charges().tds_finish("ch_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tds_finish(&self, charge_id: &str) -> PayjpResult<Charge> {
        let path = format!("/charges/{}/tds_finish", charge_id);
        self.client.post(&path, &serde_json::json!({})).await
    }

    /// List all charges.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListChargeParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let charges = client.charges().list(
    ///     ListChargeParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListChargeParams) -> PayjpResult<ListResponse<Charge>> {
        self.client.get_with_params("/charges", &params).await
    }
}
