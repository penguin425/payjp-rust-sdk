//! Example: Creating a customer with a card
//!
//! This example demonstrates creating a customer using a pre-created token.
//!
//! To get a token for testing, run:
//!   PAYJP_PUBLIC_KEY=pk_test_xxxxx PAYJP_PUBLIC_PASSWORD=password cargo run --example create_token_public
//!
//! Run with:
//!   PAYJP_SECRET_KEY=sk_test_xxxxx PAYJP_TOKEN_ID=tok_xxxxx cargo run --example create_customer

use payjp::{CreateCustomerParams, PayjpClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("PAYJP_SECRET_KEY")
        .expect("PAYJP_SECRET_KEY environment variable not set");

    let token_id = env::var("PAYJP_TOKEN_ID")
        .expect("PAYJP_TOKEN_ID environment variable not set. Run create_token_public first to get a token.");

    let client = PayjpClient::new(api_key)?;

    println!("Creating customer with token: {}", token_id);

    // Create a customer with the token
    let customer = client
        .customers()
        .create(
            CreateCustomerParams::new()
                .email("tanaka@example.com")
                .description("VIP Customer")
                .card(token_id),
        )
        .await?;

    println!("✓ Customer created successfully!");
    println!("  ID: {}", customer.id);

    if let Some(email) = &customer.email {
        println!("  Email: {}", email);
    }

    if let Some(description) = &customer.description {
        println!("  Description: {}", description);
    }

    if let Some(default_card) = &customer.default_card {
        println!(
            "  Default card: {} ending in {}",
            default_card.brand, default_card.last4
        );
    }

    Ok(())
}
