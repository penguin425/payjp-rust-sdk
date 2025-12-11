//! Plan resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::{ListParams, Metadata};
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// A plan defines the recurring billing details for subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    /// Unique identifier for the plan (prefixed with `pln_`).
    pub id: String,

    /// Object type (always "plan").
    pub object: String,

    /// Whether this plan was created in live mode.
    pub livemode: bool,

    /// Plan creation timestamp (Unix timestamp).
    pub created: i64,

    /// Amount to charge per billing interval (in smallest currency unit).
    pub amount: i64,

    /// Three-letter ISO currency code (e.g., "jpy").
    pub currency: String,

    /// Billing interval ("month" or "year").
    pub interval: PlanInterval,

    /// Plan name (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Number of trial days before first charge (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i64>,

    /// Billing day of month (1-31, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_day: Option<i32>,

    /// Set of key-value pairs for storing additional information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Billing interval for a plan.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlanInterval {
    /// Monthly billing.
    Month,

    /// Yearly billing.
    Year,
}

/// Parameters for creating a plan.
#[derive(Debug, Clone, Serialize)]
pub struct CreatePlanParams {
    /// Amount to charge per billing interval (in smallest currency unit).
    pub amount: i64,

    /// Three-letter ISO currency code (currently only "jpy" is supported).
    pub currency: String,

    /// Billing interval ("month" or "year").
    pub interval: PlanInterval,

    /// Unique plan ID (optional, auto-generated if not provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Number of trial days before first charge (0-36500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i64>,

    /// Billing day of month (1-31).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_day: Option<i32>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreatePlanParams {
    /// Create new plan parameters.
    pub fn new(amount: i64, currency: impl Into<String>, interval: PlanInterval) -> Self {
        Self {
            amount,
            currency: currency.into(),
            interval,
            id: None,
            name: None,
            trial_days: None,
            billing_day: None,
            metadata: None,
        }
    }

    /// Set a custom plan ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the plan name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the number of trial days.
    pub fn trial_days(mut self, days: i64) -> Self {
        self.trial_days = Some(days);
        self
    }

    /// Set the billing day of month.
    pub fn billing_day(mut self, day: i32) -> Self {
        self.billing_day = Some(day);
        self
    }

    /// Add metadata to the plan.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }
}

/// Parameters for updating a plan.
#[derive(Debug, Default, Clone, Serialize)]
pub struct UpdatePlanParams {
    /// Plan name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Number of trial days before first charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_days: Option<i64>,

    /// Billing day of month (1-31).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_day: Option<i32>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl UpdatePlanParams {
    /// Create new update plan parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the plan name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the number of trial days.
    pub fn trial_days(mut self, days: i64) -> Self {
        self.trial_days = Some(days);
        self
    }

    /// Set the billing day of month.
    pub fn billing_day(mut self, day: i32) -> Self {
        self.billing_day = Some(day);
        self
    }

    /// Add metadata to the plan.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }
}

/// Response from deleting a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedPlan {
    /// Plan ID.
    pub id: String,

    /// Whether the deletion was successful.
    pub deleted: bool,

    /// Whether this plan was in live mode.
    pub livemode: bool,
}

/// Service for managing plans.
pub struct PlanService<'a> {
    client: &'a PayjpClient,
}

impl<'a> PlanService<'a> {
    /// Create a new plan service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Create a new plan.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, CreatePlanParams, PlanInterval};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let plan = client.plans().create(
    ///     CreatePlanParams::new(1000, "jpy", PlanInterval::Month)
    ///         .name("Monthly Plan")
    ///         .trial_days(30)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, params: CreatePlanParams) -> PayjpResult<Plan> {
        self.client.post("/plans", &params).await
    }

    /// Retrieve a plan by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let plan = client.plans().retrieve("pln_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, plan_id: &str) -> PayjpResult<Plan> {
        let path = format!("/plans/{}", plan_id);
        self.client.get(&path).await
    }

    /// Update a plan.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, UpdatePlanParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let plan = client.plans().update(
    ///     "pln_xxxxx",
    ///     UpdatePlanParams::new().name("Updated Plan Name")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, plan_id: &str, params: UpdatePlanParams) -> PayjpResult<Plan> {
        let path = format!("/plans/{}", plan_id);
        self.client.post(&path, &params).await
    }

    /// Delete a plan.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let deleted = client.plans().delete("pln_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, plan_id: &str) -> PayjpResult<DeletedPlan> {
        let path = format!("/plans/{}", plan_id);
        self.client.delete(&path).await
    }

    /// List all plans.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let plans = client.plans().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<Plan>> {
        self.client.get_with_params("/plans", &params).await
    }
}
