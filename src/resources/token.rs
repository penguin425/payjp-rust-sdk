//! Token resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::resources::card::Card;
use serde::{Deserialize, Serialize};

/// A token represents a card that can be used to create a charge or customer.
/// Tokens are one-time use and expire after a short period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    /// Unique identifier for the token (prefixed with `tok_`).
    pub id: String,

    /// Object type (always "token").
    pub object: String,

    /// Whether this token was created in live mode.
    pub livemode: bool,

    /// Token creation timestamp (Unix timestamp).
    pub created: i64,

    /// Whether this token has been used.
    pub used: bool,

    /// Card information associated with this token.
    pub card: Card,
}

/// Raw card details for creating a token (server-side only for testing).
/// In production, tokens should be created client-side using PAY.JP.js.
#[derive(Debug, Default, Clone, Serialize)]
pub struct CardDetails {
    /// Card number (without spaces or hyphens).
    #[serde(rename = "card[number]")]
    pub number: String,

    /// Card expiration month (1-12).
    #[serde(rename = "card[exp_month]")]
    pub exp_month: i32,

    /// Card expiration year (4 digits).
    #[serde(rename = "card[exp_year]")]
    pub exp_year: i32,

    /// Card CVC/CVV code.
    #[serde(rename = "card[cvc]")]
    pub cvc: String,

    /// Cardholder name (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "card[name]")]
    pub name: Option<String>,

    /// Address line 1 (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "card[address_line1]")]
    pub address_line1: Option<String>,

    /// Address line 2 (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "card[address_line2]")]
    pub address_line2: Option<String>,

    /// Address city (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "card[address_city]")]
    pub address_city: Option<String>,

    /// Address state/prefecture (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "card[address_state]")]
    pub address_state: Option<String>,

    /// Address ZIP/postal code (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "card[address_zip]")]
    pub address_zip: Option<String>,

    /// Address country (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "card[country]")]
    pub country: Option<String>,

    /// Email address (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "card[email]")]
    pub email: Option<String>,

    /// Phone number (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "card[phone]")]
    pub phone: Option<String>,
}

impl CardDetails {
    /// Create new card details for tokenization.
    ///
    /// **WARNING**: This should only be used for testing with test cards.
    /// In production, use PAY.JP.js to create tokens client-side.
    pub fn new(
        number: impl Into<String>,
        exp_month: i32,
        exp_year: i32,
        cvc: impl Into<String>,
    ) -> Self {
        Self {
            number: number.into(),
            exp_month,
            exp_year,
            cvc: cvc.into(),
            ..Default::default()
        }
    }

    /// Set the cardholder name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the email address.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
}

/// Parameters for creating a token.
#[derive(Debug, Default, Clone, Serialize)]
pub struct CreateTokenParams {
    /// Raw card details (server-side only for testing).
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub card: Option<CardDetails>,
}

impl CreateTokenParams {
    /// Create token parameters with card details.
    ///
    /// **WARNING**: This should only be used for testing with test cards.
    /// In production, use PAY.JP.js to create tokens client-side.
    pub fn from_card(card: CardDetails) -> Self {
        Self { card: Some(card) }
    }
}

/// Service for managing tokens.
pub struct TokenService<'a> {
    client: &'a PayjpClient,
}

impl<'a> TokenService<'a> {
    /// Create a new token service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Create a new token.
    ///
    /// **WARNING**: Creating tokens with raw card data should only be done
    /// for testing purposes. In production, use PAY.JP.js to create tokens
    /// client-side to avoid handling sensitive card data on your server.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, CreateTokenParams, CardDetails};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// // Test card number
    /// let card = CardDetails::new("4242424242424242", 12, 2025, "123")
    ///     .name("山田太郎");
    ///
    /// let token = client.tokens().create(
    ///     CreateTokenParams::from_card(card)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, params: CreateTokenParams) -> PayjpResult<Token> {
        self.client.post("/tokens", &params).await
    }

    /// Retrieve a token by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let token = client.tokens().retrieve("tok_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, token_id: &str) -> PayjpResult<Token> {
        let path = format!("/tokens/{}", token_id);
        self.client.get(&path).await
    }

    /// Finish 3D Secure authentication for a token.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let token = client.tokens().tds_finish("tok_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn tds_finish(&self, token_id: &str) -> PayjpResult<Token> {
        let path = format!("/tokens/{}/tds_finish", token_id);
        self.client.post(&path, &serde_json::json!({})).await
    }
}

/// Service for managing tokens with a public key (client-side).
///
/// This service can only create tokens using a public key. It's designed for
/// client-side token creation to avoid sending raw card data to your server.
pub struct PublicTokenService<'a> {
    client: &'a crate::client::PayjpPublicClient,
}

impl<'a> PublicTokenService<'a> {
    /// Create a new public token service.
    pub(crate) fn new(client: &'a crate::client::PayjpPublicClient) -> Self {
        Self { client }
    }

    /// Create a new token with a public key.
    ///
    /// This is the recommended way to create tokens - use a public key (pk_test_ or pk_live_)
    /// to tokenize card data without sending it to your server.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpPublicClient, CreateTokenParams, CardDetails};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PayjpPublicClient::new("pk_test_xxxxx")?;
    ///
    /// // Test card number
    /// let card = CardDetails::new("4242424242424242", 12, 2030, "123")
    ///     .name("Taro Yamada");
    ///
    /// let token = client.tokens().create(
    ///     CreateTokenParams::from_card(card)
    /// ).await?;
    ///
    /// println!("Token ID: {}", token.id);
    /// // Send token.id to your server
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(&self, params: CreateTokenParams) -> PayjpResult<Token> {
        self.client.post("/tokens", &params).await
    }
}
