//! Card resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::{ListParams, Metadata};
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// A card object represents a credit or debit card associated with a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// Unique identifier for the card (prefixed with `car_`).
    pub id: String,

    /// Object type (always "card").
    pub object: String,

    /// Whether this card was created in live mode.
    pub livemode: bool,

    /// Card creation timestamp (Unix timestamp).
    pub created: i64,

    /// Customer ID this card belongs to (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Card brand (e.g., "Visa", "MasterCard", "JCB", "American Express", "Diners Club", "Discover").
    pub brand: String,

    /// Card CVC check result (e.g., "passed", "failed", "unchecked").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,

    /// Card expiration month (1-12).
    pub exp_month: i32,

    /// Card expiration year (4 digits).
    pub exp_year: i32,

    /// Fingerprint for duplicate detection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    /// Last 4 digits of the card number.
    pub last4: String,

    /// Cardholder name (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Address line 1 (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,

    /// Address line 2 (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,

    /// Address city (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,

    /// Address state/prefecture (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,

    /// Address ZIP/postal code (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,

    /// Address ZIP check result (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip_check: Option<String>,

    /// Address country (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// 3D Secure support status (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure_status: Option<CardThreeDSecureStatus>,

    /// Email address (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Phone number (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// 3D Secure status for a card.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CardThreeDSecureStatus {
    /// 3D Secure verification was not performed.
    Unverified,

    /// 3D Secure verification was successful.
    Verified,

    /// 3D Secure verification was attempted.
    Attempted,

    /// 3D Secure verification failed.
    Failed,

    /// An error occurred during 3D Secure verification.
    Error,
}

/// Parameters for creating a card.
#[derive(Debug, Default, Clone, Serialize)]
pub struct CreateCardParams {
    /// Card token ID or raw card details token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Whether to set this card as the default for the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

impl CreateCardParams {
    /// Create new card parameters with a token.
    pub fn new(card_token: impl Into<String>) -> Self {
        Self {
            card: Some(card_token.into()),
            ..Default::default()
        }
    }

    /// Add metadata to the card.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }

    /// Set this card as the default for the customer.
    pub fn set_default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }
}

/// Parameters for updating a card.
#[derive(Debug, Default, Clone, Serialize)]
pub struct UpdateCardParams {
    /// Card expiration month (1-12).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i32>,

    /// Card expiration year (4 digits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i32>,

    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Address line 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,

    /// Address line 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,

    /// Address city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,

    /// Address state/prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,

    /// Address ZIP/postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,

    /// Address country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Set of key-value pairs for storing additional information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl UpdateCardParams {
    /// Create new update card parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the expiration date.
    pub fn expiration(mut self, month: i32, year: i32) -> Self {
        self.exp_month = Some(month);
        self.exp_year = Some(year);
        self
    }

    /// Set the cardholder name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Add metadata to the card.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(Default::default)
            .insert(key.into(), value.into());
        self
    }
}

/// Service for managing cards associated with a customer.
pub struct CardService<'a> {
    client: &'a PayjpClient,
    customer_id: String,
}

impl<'a> CardService<'a> {
    /// Create a new card service for a specific customer.
    pub(crate) fn new(client: &'a PayjpClient, customer_id: String) -> Self {
        Self {
            client,
            customer_id,
        }
    }

    /// Create a new card for the customer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, CreateCardParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let card = client.customer("cus_xxxxx").cards().create(
    ///     CreateCardParams::new("tok_xxxxx")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, params: CreateCardParams) -> PayjpResult<Card> {
        let path = format!("/customers/{}/cards", self.customer_id);
        self.client.post(&path, &params).await
    }

    /// Retrieve a card by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let card = client.customer("cus_xxxxx").cards().retrieve("car_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, card_id: &str) -> PayjpResult<Card> {
        let path = format!("/customers/{}/cards/{}", self.customer_id, card_id);
        self.client.get(&path).await
    }

    /// Update a card.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, UpdateCardParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let card = client.customer("cus_xxxxx").cards().update(
    ///     "car_xxxxx",
    ///     UpdateCardParams::new().name("山田太郎")
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self, card_id: &str, params: UpdateCardParams) -> PayjpResult<Card> {
        let path = format!("/customers/{}/cards/{}", self.customer_id, card_id);
        self.client.post(&path, &params).await
    }

    /// Delete a card.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let deleted = client.customer("cus_xxxxx").cards().delete("car_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, card_id: &str) -> PayjpResult<DeletedCard> {
        let path = format!("/customers/{}/cards/{}", self.customer_id, card_id);
        self.client.delete(&path).await
    }

    /// List all cards for the customer.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let cards = client.customer("cus_xxxxx").cards().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<Card>> {
        let path = format!("/customers/{}/cards", self.customer_id);
        self.client.get_with_params(&path, &params).await
    }
}

/// Response from deleting a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedCard {
    /// Unique identifier for the card.
    pub id: String,

    /// Whether the deletion was successful.
    pub deleted: bool,

    /// Whether this card was in live mode.
    pub livemode: bool,
}
