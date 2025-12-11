//! Subscription resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::{ListParams, Metadata};
use crate::resources::plan::Plan;
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// A subscription represents a recurring payment for a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Unique identifier for the subscription (prefixed with `sub_`).
    pub id: String,

    /// Object type (always "subscription").
    pub object: String,

    /// Whether this subscription was created in live mode.
    pub livemode: bool,

    /// Subscription creation timestamp (Unix timestamp).
    pub created: i64,

    /// Customer ID associated with this subscription.
    pub customer: String,

    /// Plan details for this subscription.
    pub plan: Plan,

    /// Subscription status.
    pub status: SubscriptionStatus,

    /// Timestamp when the subscription started (Unix timestamp).
    pub start: i64,

    /// Timestamp when the trial period ends (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,

    /// Timestamp when the subscription was paused (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused_at: Option<i64>,

    /// Timestamp when the subscription was canceled (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,

    /// Timestamp when the subscription ends/ended (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<i64>,

    /// Timestamp when the current period started (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<i64>,

    /// Timestamp when the subscription was resumed (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumed_at: Option<i64>,

    /// Whether to prorate when updating the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,

    /// Set of key-value pairs for storing additional information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Status of a subscription.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    /// Subscription is active and will be charged.
    Active,

    /// Subscription is in trial period.
    Trial,

    /// Subscription has been canceled.
    Canceled,

    /// Subscription has been paused.
    Paused,
}

/// Parameters for creating a subscription.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSubscriptionParams {
    /// Customer ID.
    pub customer: String,

    /// Plan ID.
    pub plan: String,

    /// Trial end date as Unix timestamp (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,

    /// Whether to prorate charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreateSubscriptionParams {
    /// Create new subscription parameters.
    pub fn new(customer: impl Into<String>, plan: impl Into<String>) -> Self {
        Self {
            customer: customer.into(),
            plan: plan.into(),
            trial_end: None,
            prorate: None,
            metadata: None,
        }
    }

    /// Set the trial end timestamp.
    pub fn trial_end(mut self, timestamp: i64) -> Self {
        self.trial_end = Some(timestamp);
        self
    }

    /// Set whether to prorate charges.
    pub fn prorate(mut self, prorate: bool) -> Self {
        self.prorate = Some(prorate);
        self
    }

    /// Add metadata to the subscription.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }
}

/// Parameters for updating a subscription.
#[derive(Debug, Default, Clone, Serialize)]
pub struct UpdateSubscriptionParams {
    /// New plan ID (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    /// Trial end date as Unix timestamp (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<i64>,

    /// Whether to prorate charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl UpdateSubscriptionParams {
    /// Create new update subscription parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a new plan.
    pub fn plan(mut self, plan: impl Into<String>) -> Self {
        self.plan = Some(plan.into());
        self
    }

    /// Set the trial end timestamp.
    pub fn trial_end(mut self, timestamp: i64) -> Self {
        self.trial_end = Some(timestamp);
        self
    }

    /// Set whether to prorate charges.
    pub fn prorate(mut self, prorate: bool) -> Self {
        self.prorate = Some(prorate);
        self
    }

    /// Add metadata to the subscription.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }
}

/// Parameters for pausing a subscription.
#[derive(Debug, Default, Clone, Serialize)]
pub struct PauseSubscriptionParams {}

impl PauseSubscriptionParams {
    /// Create new pause subscription parameters.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Parameters for resuming a subscription.
#[derive(Debug, Default, Clone, Serialize)]
pub struct ResumeSubscriptionParams {
    /// Whether to charge for the period during which the subscription was paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
}

impl ResumeSubscriptionParams {
    /// Create new resume subscription parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set whether to prorate charges for the paused period.
    pub fn prorate(mut self, prorate: bool) -> Self {
        self.prorate = Some(prorate);
        self
    }
}

/// Parameters for canceling a subscription.
#[derive(Debug, Default, Clone, Serialize)]
pub struct CancelSubscriptionParams {}

impl CancelSubscriptionParams {
    /// Create new cancel subscription parameters.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Response from deleting a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedSubscription {
    /// Subscription ID.
    pub id: String,

    /// Whether the deletion was successful.
    pub deleted: bool,

    /// Whether this subscription was in live mode.
    pub livemode: bool,
}

/// Service for managing subscriptions.
pub struct SubscriptionService<'a> {
    client: &'a PayjpClient,
}

impl<'a> SubscriptionService<'a> {
    /// Create a new subscription service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Create a new subscription.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, CreateSubscriptionParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let subscription = client.subscriptions().create(
    ///     CreateSubscriptionParams::new("cus_xxxxx", "pln_xxxxx")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, params: CreateSubscriptionParams) -> PayjpResult<Subscription> {
        self.client.post("/subscriptions", &params).await
    }

    /// Retrieve a subscription by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let subscription = client.subscriptions().retrieve("sub_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, subscription_id: &str) -> PayjpResult<Subscription> {
        let path = format!("/subscriptions/{}", subscription_id);
        self.client.get(&path).await
    }

    /// Update a subscription.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, UpdateSubscriptionParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let subscription = client.subscriptions().update(
    ///     "sub_xxxxx",
    ///     UpdateSubscriptionParams::new().plan("pln_new")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        subscription_id: &str,
        params: UpdateSubscriptionParams,
    ) -> PayjpResult<Subscription> {
        let path = format!("/subscriptions/{}", subscription_id);
        self.client.post(&path, &params).await
    }

    /// Pause a subscription.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, PauseSubscriptionParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let subscription = client.subscriptions().pause(
    ///     "sub_xxxxx",
    ///     PauseSubscriptionParams::new()
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pause(
        &self,
        subscription_id: &str,
        params: PauseSubscriptionParams,
    ) -> PayjpResult<Subscription> {
        let path = format!("/subscriptions/{}/pause", subscription_id);
        self.client.post(&path, &params).await
    }

    /// Resume a paused subscription.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ResumeSubscriptionParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let subscription = client.subscriptions().resume(
    ///     "sub_xxxxx",
    ///     ResumeSubscriptionParams::new()
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn resume(
        &self,
        subscription_id: &str,
        params: ResumeSubscriptionParams,
    ) -> PayjpResult<Subscription> {
        let path = format!("/subscriptions/{}/resume", subscription_id);
        self.client.post(&path, &params).await
    }

    /// Cancel a subscription.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, CancelSubscriptionParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let subscription = client.subscriptions().cancel(
    ///     "sub_xxxxx",
    ///     CancelSubscriptionParams::new()
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel(
        &self,
        subscription_id: &str,
        params: CancelSubscriptionParams,
    ) -> PayjpResult<Subscription> {
        let path = format!("/subscriptions/{}/cancel", subscription_id);
        self.client.post(&path, &params).await
    }

    /// Delete a subscription.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let deleted = client.subscriptions().delete("sub_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, subscription_id: &str) -> PayjpResult<DeletedSubscription> {
        let path = format!("/subscriptions/{}", subscription_id);
        self.client.delete(&path).await
    }

    /// List all subscriptions.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let subscriptions = client.subscriptions().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<Subscription>> {
        self.client.get_with_params("/subscriptions", &params).await
    }
}
