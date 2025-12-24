//! # PAY.JP Rust SDK
//!
//! This crate provides a type-safe Rust interface to the PAY.JP payment API.
//!
//! ## Quick Start
//!
//! ```no_run
//! use payjp::{PayjpClient, CreateChargeParams};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = PayjpClient::new("sk_test_xxxxx")?;
//!
//!     // Create a charge
//!     let charge = client.charges().create(
//!         CreateChargeParams::new(1000, "jpy")
//!             .card("tok_xxxxx")
//!             .description("Test charge")
//!     ).await?;
//!
//!     println!("Charge ID: {}", charge.id);
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **Type-safe API**: Full type safety with Rust's type system
//! - **Async/await**: Built on tokio for async operations
//! - **Rate limiting**: Automatic retry with exponential backoff
//! - **Comprehensive**: Supports all PAY.JP resources
//! - **Platform API**: Support for multi-tenant applications
//!
//! ## Resources
//!
//! The SDK supports the following PAY.JP resources:
//!
//! - **Charges** - Create and manage payments
//! - **Customers** - Manage customer accounts
//! - **Cards** - Manage customer payment methods
//! - **Tokens** - Tokenize card information
//! - **Plans** - Define recurring billing plans
//! - **Subscriptions** - Manage recurring payments
//! - **Transfers** - View payout information
//! - **Events** - Retrieve webhook events
//! - **Statements** - Access transaction statements
//! - **Balances** - View account balances
//! - **Terms** - View aggregation periods
//! - **Account** - Retrieve account information
//! - **3D Secure** - Handle 3D Secure authentication
//! - **Tenants** (Platform API) - Manage sub-merchants
//! - **Tenant Transfers** (Platform API) - View tenant payouts

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod client;
pub mod error;
pub mod params;
pub mod resources;
pub mod response;

// Re-export main types
pub use client::{ClientOptions, PayjpClient, PayjpPublicClient, DEFAULT_BASE_URL};
pub use error::{ApiError, CardError, PayjpError, PayjpResult};
pub use params::{ListParams, Metadata};
pub use response::ListResponse;

// Re-export resource types
pub use resources::{
    Account, AccountService, Balance, BalanceService, CancelSubscriptionParams, CaptureParams,
    Card, CardDetails, CardService, CardThreeDSecureStatus, Charge, ChargeService,
    CreateCardParams, CreateChargeParams, CreateCustomerParams, CreatePlanParams,
    CreateSubscriptionParams, CreateThreeDSecureRequestParams, CreateTokenParams, Customer,
    CustomerService, Event, EventData, EventService, EventType, ListChargeParams,
    PauseSubscriptionParams, Plan, PlanInterval, PlanService, ReauthParams, RefundParams,
    ResumeSubscriptionParams, Statement, StatementService, Subscription, SubscriptionService,
    SubscriptionStatus, Term, TermService, ThreeDSecureRequest, ThreeDSecureRequestService,
    ThreeDSecureStatus, Token, TokenService, PublicTokenService, Transfer, TransferService, UpdateCardParams,
    UpdateChargeParams, UpdateCustomerParams, UpdatePlanParams, UpdateSubscriptionParams,
};

// Re-export platform types
pub use resources::platform::{
    CreateTenantParams, Tenant, TenantService, TenantTransfer, TenantTransferService,
    UpdateTenantParams,
};

// Add service accessor methods to PayjpClient
impl PayjpClient {
    /// Access the charges service.
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
    pub fn charges(&self) -> resources::ChargeService<'_> {
        resources::ChargeService::new(self)
    }

    /// Access the customers service.
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
    pub fn customers(&self) -> resources::CustomerService<'_> {
        resources::CustomerService::new(self)
    }

    /// Access a specific customer and its related resources.
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
    pub fn customer(&self, customer_id: impl Into<String>) -> resources::customer::CustomerWrapper<'_> {
        resources::customer::CustomerWrapper::new(self, customer_id.into())
    }

    /// Access the tokens service.
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
    pub fn tokens(&self) -> resources::TokenService<'_> {
        resources::TokenService::new(self)
    }

    /// Access the plans service.
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
    pub fn plans(&self) -> resources::PlanService<'_> {
        resources::PlanService::new(self)
    }

    /// Access the subscriptions service.
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
    pub fn subscriptions(&self) -> resources::SubscriptionService<'_> {
        resources::SubscriptionService::new(self)
    }

    /// Access the transfers service.
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
    pub fn transfers(&self) -> resources::TransferService<'_> {
        resources::TransferService::new(self)
    }

    /// Access the events service.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let event = client.events().retrieve("evnt_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn events(&self) -> resources::EventService<'_> {
        resources::EventService::new(self)
    }

    /// Access the account service.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let account = client.account().retrieve().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn account(&self) -> resources::AccountService<'_> {
        resources::AccountService::new(self)
    }

    /// Access the statements service.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let statement = client.statements().retrieve("st_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn statements(&self) -> resources::StatementService<'_> {
        resources::StatementService::new(self)
    }

    /// Access the balances service.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let balance = client.balances().retrieve("ba_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn balances(&self) -> resources::BalanceService<'_> {
        resources::BalanceService::new(self)
    }

    /// Access the terms service.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let term = client.terms().retrieve("tm_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn terms(&self) -> resources::TermService<'_> {
        resources::TermService::new(self)
    }

    /// Access the 3D Secure requests service.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let tds_request = client.three_d_secure_requests().retrieve("tdsr_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn three_d_secure_requests(&self) -> resources::ThreeDSecureRequestService<'_> {
        resources::ThreeDSecureRequestService::new(self)
    }

    /// Access the tenants service (Platform API).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx")?;
    /// let tenant = client.tenants().retrieve("ten_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tenants(&self) -> resources::platform::TenantService<'_> {
        resources::platform::TenantService::new(self)
    }

    /// Access the tenant transfers service (Platform API).
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
    pub fn tenant_transfers(&self) -> resources::platform::TenantTransferService<'_> {
        resources::platform::TenantTransferService::new(self)
    }
}

// Add service accessor methods to PayjpPublicClient
impl PayjpPublicClient {
    /// Access the tokens service (public key).
    ///
    /// This is the only service available with a public key. Use this to create
    /// tokens client-side without sending raw card data to your server.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpPublicClient, CreateTokenParams, CardDetails};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PayjpPublicClient::new("pk_test_xxxxx")?;
    ///
    /// let card = CardDetails::new("4242424242424242", 12, 2030, "123");
    /// let token = client.tokens().create(
    ///     CreateTokenParams::from_card(card)
    /// ).await?;
    ///
    /// // Send token.id to your server
    /// println!("Token ID: {}", token.id);
    /// # Ok(())
    /// # }
    /// ```
    pub fn tokens(&self) -> resources::token::PublicTokenService<'_> {
        resources::token::PublicTokenService::new(self)
    }
}
