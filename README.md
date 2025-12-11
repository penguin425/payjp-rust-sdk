# PAY.JP Rust SDK

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Rust SDK for [PAY.JP](https://pay.jp), a Japanese payment platform. This library provides type-safe, async/await interfaces to all PAY.JP APIs.

## Features

- 🦀 **Type-safe** - Full type safety with Rust's type system
- ⚡ **Async/await** - Built on tokio for efficient async operations
- 🔄 **Automatic retries** - Exponential backoff with jitter for rate limiting
- 📦 **Comprehensive** - Supports all PAY.JP resources and operations
- 🏢 **Platform API** - Full support for multi-tenant applications
- 🔒 **3D Secure** - Complete 3D Secure authentication support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
payjp = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use payjp::{PayjpClient, CreateChargeParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with your secret key
    let client = PayjpClient::new("sk_test_xxxxx");

    // Create a charge
    let charge = client.charges().create(
        CreateChargeParams::new(1000, "jpy")
            .card("tok_xxxxx")  // Token created client-side
            .description("商品購入")
    ).await?;

    println!("Charge created: {}", charge.id);
    Ok(())
}
```

## Supported Resources

### Core Resources

- **Charges** - Create and manage payments
- **Customers** - Manage customer accounts
- **Cards** - Manage customer payment methods
- **Tokens** - Tokenize card information securely
- **Plans** - Define recurring billing plans
- **Subscriptions** - Manage recurring payments

### Additional Resources

- **Transfers** - View payout information
- **Events** - Retrieve webhook events
- **Statements** - Access transaction statements
- **Balances** - View account balances
- **Terms** - View aggregation periods
- **Account** - Retrieve account information
- **3D Secure** - Handle 3D Secure authentication

### Platform API

- **Tenants** - Manage sub-merchants
- **Tenant Transfers** - View tenant payouts

## Usage Examples

### Creating a Customer

```rust
use payjp::{PayjpClient, CreateCustomerParams};

let customer = client.customers().create(
    CreateCustomerParams::new()
        .email("customer@example.com")
        .card("tok_xxxxx")
        .metadata("customer_type", "premium")
).await?;
```

### Creating a Subscription

```rust
use payjp::{CreatePlanParams, CreateSubscriptionParams, PlanInterval};

// Create a plan
let plan = client.plans().create(
    CreatePlanParams::new(1000, "jpy", PlanInterval::Month)
        .name("月額プラン")
        .trial_days(7)
).await?;

// Subscribe a customer to the plan
let subscription = client.subscriptions().create(
    CreateSubscriptionParams::new(&customer.id, &plan.id)
).await?;
```

### Managing Cards

```rust
use payjp::{CreateCardParams, UpdateCardParams};

// Add a card to a customer
let card = client.customer(&customer_id)
    .cards()
    .create(CreateCardParams::new("tok_xxxxx"))
    .await?;

// Update card details
let updated = client.customer(&customer_id)
    .cards()
    .update(&card.id, UpdateCardParams::new().name("新しい名前"))
    .await?;

// List all cards
let cards = client.customer(&customer_id)
    .cards()
    .list(Default::default())
    .await?;
```

### Refunding a Charge

```rust
use payjp::RefundParams;

let refunded_charge = client.charges().refund(
    &charge.id,
    RefundParams::new()
        .amount(500)  // Partial refund
        .reason("顧客都合")
).await?;
```

### Working with Metadata

```rust
use payjp::CreateChargeParams;

let charge = client.charges().create(
    CreateChargeParams::new(1000, "jpy")
        .card("tok_xxxxx")
        .metadata("order_id", "12345")
        .metadata("customer_name", "山田太郎")
        .metadata("product", "商品A")
).await?;
```

### Listing Resources with Pagination

```rust
use payjp::ListParams;

let charges = client.charges().list(
    ListParams::new()
        .limit(20)
        .offset(0)
).await?;

for charge in charges.data {
    println!("Charge: {} - ¥{}", charge.id, charge.amount);
}
```

### 3D Secure Authentication

```rust
use payjp::CreateThreeDSecureRequestParams;

// Create a 3DS request for a token
let tds_request = client.three_d_secure_requests().create(
    CreateThreeDSecureRequestParams::new("token", &token.id)
        .return_url("https://example.com/callback")
).await?;

// User completes authentication...

// Finish 3DS authentication
let completed_token = client.tokens().tds_finish(&token.id).await?;
```

### Platform API - Managing Tenants

```rust
use payjp::CreateTenantParams;

let tenant = client.tenants().create(
    CreateTenantParams::new()
        .name("サブマーチャント")
        .platform_fee_rate("0.10")  // 10% platform fee
).await?;

// Create a charge for a tenant
let charge = client.charges().create(
    CreateChargeParams::new(1000, "jpy")
        .card("tok_xxxxx")
        .tenant(&tenant.id)
        .platform_fee(100)
).await?;
```

## Configuration

### Custom Client Options

```rust
use payjp::{PayjpClient, ClientOptions};
use std::time::Duration;

let options = ClientOptions::new()
    .timeout(Duration::from_secs(60))
    .max_retry(5)
    .retry_initial_delay(Duration::from_millis(500))
    .retry_max_delay(Duration::from_secs(30));

let client = PayjpClient::with_options("sk_test_xxxxx", options);
```

### Rate Limiting

The SDK automatically handles rate limiting with exponential backoff and jitter. When a `429 Too Many Requests` response is received, the SDK will retry the request with increasing delays.

PAY.JP rate limits:

| Mode | Zone | Rate (req/sec) |
|------|------|----------------|
| Live | pk | 10 |
| Live | payment | 14 |
| Live | sk | 30 |
| Test | pk | 2 |
| Test | payment | 2 |
| Test | sk | 2 |

## Error Handling

```rust
use payjp::PayjpError;

match client.charges().retrieve("ch_invalid").await {
    Ok(charge) => println!("Charge: {:?}", charge),
    Err(PayjpError::Api(api_err)) => {
        eprintln!("API Error: {}", api_err.message);
        eprintln!("Error code: {:?}", api_err.code);
    }
    Err(PayjpError::Network(net_err)) => {
        eprintln!("Network error: {}", net_err);
    }
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Security Considerations

### Never Send Raw Card Data

**⚠️ IMPORTANT**: Never send raw card data to your server. Always use PAY.JP.js to create tokens client-side.

The `CardDetails` type is provided only for testing purposes with test cards. In production:

1. Use PAY.JP.js on your frontend to tokenize card data
2. Send only the token ID to your server
3. Use the token ID to create charges or customers

```javascript
// Frontend (JavaScript)
const payjp = Payjp('pk_test_xxxxx');
const token = await payjp.createToken(cardElement);
// Send token.id to your server
```

```rust
// Backend (Rust)
let charge = client.charges().create(
    CreateChargeParams::new(1000, "jpy")
        .card(&token_id)  // Use token from frontend
).await?;
```

### Amount Limits

- Minimum: ¥50
- Maximum: ¥9,999,999

### Refund Period

Refunds must be processed within 180 days of the original charge.

### Authorization Hold Period

Authorization holds expire after 1-60 days (default: 7 days).

## Running Examples

Set your PAY.JP secret key as an environment variable:

```bash
export PAYJP_SECRET_KEY=sk_test_xxxxx
```

Run an example:

```bash
cargo run --example create_charge
cargo run --example create_customer
cargo run --example subscription
cargo run --example three_d_secure
```

## Testing

```bash
# Run unit tests
cargo test

# Run with all features
cargo test --all-features
```

## Documentation

- [PAY.JP Official API Documentation](https://pay.jp/docs/api/)
- [API Reference (docs.rs)](https://docs.rs/payjp) (Coming soon)

## Resources

- [PAY.JP Website](https://pay.jp)
- [PAY.JP Dashboard](https://pay.jp/dashboard)
- [Test Cards](https://pay.jp/docs/testcard)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

- Built with [reqwest](https://github.com/seanmonstar/reqwest) for HTTP client
- Uses [serde](https://github.com/serde-rs/serde) for JSON serialization
- Inspired by official PAY.JP SDKs for other languages

## Disclaimer

This is an unofficial SDK and is not affiliated with or endorsed by PAY.JP.
