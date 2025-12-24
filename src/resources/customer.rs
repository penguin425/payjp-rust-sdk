//! Customer resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::{ListParams, Metadata};
use crate::resources::card::{Card, CardService};
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// Represents either a Card object or a card ID string.
///
/// PAY.JP API returns card IDs by default, but can return full Card objects
/// when using the `expand` parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CardOrId {
    /// Full Card object (when expanded).
    Card(Card),
    /// Card ID string.
    Id(String),
}

/// A customer represents a buyer who can be charged multiple times.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// Unique identifier for the customer (prefixed with `cus_`).
    pub id: String,

    /// Object type (always "customer").
    pub object: String,

    /// Whether this customer was created in live mode.
    pub livemode: bool,

    /// Customer creation timestamp (Unix timestamp).
    pub created: i64,

    /// Customer's default card (optional).
    /// Can be either a card ID string or a full Card object if expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_card: Option<CardOrId>,

    /// Customer's email address (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Customer description (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set of key-value pairs for storing additional information (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// List of subscriptions for this customer (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<ListResponse<crate::resources::subscription::Subscription>>,

    /// Cards associated with this customer (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards: Option<ListResponse<Card>>,
}

/// Parameters for creating a customer.
#[derive(Debug, Default, Clone, Serialize)]
pub struct CreateCustomerParams {
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Card token ID to add as the default card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl CreateCustomerParams {
    /// Create new customer parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the email address.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the default card using a token.
    pub fn card(mut self, card: impl Into<String>) -> Self {
        self.card = Some(card.into());
        self
    }

    /// Add metadata to the customer.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }
}

/// Parameters for updating a customer.
#[derive(Debug, Default, Clone, Serialize)]
pub struct UpdateCustomerParams {
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Card token ID to set as the default card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_card: Option<String>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl UpdateCustomerParams {
    /// Create new update customer parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the email address.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Set the description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the default card.
    pub fn default_card(mut self, card: impl Into<String>) -> Self {
        self.default_card = Some(card.into());
        self
    }

    /// Add metadata to the customer.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }
}

/// Response from deleting a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedCustomer {
    /// Customer ID.
    pub id: String,

    /// Whether the deletion was successful.
    pub deleted: bool,

    /// Whether this customer was in live mode.
    pub livemode: bool,
}

/// Service for managing customers.
pub struct CustomerService<'a> {
    client: &'a PayjpClient,
}

impl<'a> CustomerService<'a> {
    /// Create a new customer service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Create a new customer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, CreateCustomerParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let customer = client.customers().create(
    ///     CreateCustomerParams::new()
    ///         .email("customer@example.com")
    ///         .card("tok_xxxxx")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, params: CreateCustomerParams) -> PayjpResult<Customer> {
        self.client.post("/customers", &params).await
    }

    /// Retrieve a customer by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let customer = client.customers().retrieve("cus_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, customer_id: &str) -> PayjpResult<Customer> {
        let path = format!("/customers/{}", customer_id);
        self.client.get(&path).await
    }

    /// Update a customer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, UpdateCustomerParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let customer = client.customers().update(
    ///     "cus_xxxxx",
    ///     UpdateCustomerParams::new().email("new@example.com")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        customer_id: &str,
        params: UpdateCustomerParams,
    ) -> PayjpResult<Customer> {
        let path = format!("/customers/{}", customer_id);
        self.client.post(&path, &params).await
    }

    /// Delete a customer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let deleted = client.customers().delete("cus_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, customer_id: &str) -> PayjpResult<DeletedCustomer> {
        let path = format!("/customers/{}", customer_id);
        self.client.delete(&path).await
    }

    /// List all customers.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let customers = client.customers().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<Customer>> {
        self.client.get_with_params("/customers", &params).await
    }
}

/// Wrapper for accessing a specific customer and its related resources.
pub struct CustomerWrapper<'a> {
    client: &'a PayjpClient,
    customer_id: String,
}

impl<'a> CustomerWrapper<'a> {
    /// Create a new customer wrapper.
    pub(crate) fn new(client: &'a PayjpClient, customer_id: String) -> Self {
        Self {
            client,
            customer_id,
        }
    }

    /// Get the customer ID.
    pub fn id(&self) -> &str {
        &self.customer_id
    }

    /// Access the cards service for this customer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, CreateCardParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let card = client.customer("cus_xxxxx")
    ///     .cards()
    ///     .create(CreateCardParams::new("tok_xxxxx"))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cards(&self) -> CardService<'_> {
        CardService::new(self.client, self.customer_id.clone())
    }

    /// Retrieve the customer details.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let customer = client.customer("cus_xxxxx").retrieve().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self) -> PayjpResult<Customer> {
        let path = format!("/customers/{}", self.customer_id);
        self.client.get(&path).await
    }

    /// Update the customer.
    pub async fn update(&self, params: UpdateCustomerParams) -> PayjpResult<Customer> {
        let path = format!("/customers/{}", self.customer_id);
        self.client.post(&path, &params).await
    }

    /// Delete the customer.
    pub async fn delete(&self) -> PayjpResult<DeletedCustomer> {
        let path = format!("/customers/{}", self.customer_id);
        self.client.delete(&path).await
    }
}
