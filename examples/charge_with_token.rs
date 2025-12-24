//! Example: Creating a charge with an existing token
//!
//! This example demonstrates creating a charge using a pre-created token.
//! This approach avoids the need to send raw card data to the API.
//!
//! To get a token for testing:
//! 1. Go to PAY.JP dashboard: https://pay.jp/d/test/tokens
//! 2. Click "Create test token" or use PAY.JP.js in a browser
//! 3. Copy the token ID (starts with "tok_")
//!
//! Run with:
//!   PAYJP_SECRET_KEY=sk_test_xxxxx PAYJP_TOKEN_ID=tok_xxxxx cargo run --example charge_with_token

use payjp::{CreateChargeParams, PayjpClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("PAYJP_SECRET_KEY")
        .expect("PAYJP_SECRET_KEY environment variable not set");

    // Get token ID from environment variable
    let token_id = env::var("PAYJP_TOKEN_ID")
        .expect("PAYJP_TOKEN_ID environment variable not set. Get a token from https://pay.jp/d/test/tokens");

    let client = PayjpClient::new(api_key)?;

    println!("Creating charge with token: {}", token_id);

    // Create a charge using the existing token
    let charge = client
        .charges()
        .create(
            CreateChargeParams::new(1000, "jpy")
                .card(token_id)
                .description("Test payment"),
        )
        .await?;

    println!("✓ Charge created successfully!");
    println!("  ID: {}", charge.id);
    println!("  Amount: ¥{}", charge.amount);
    println!("  Paid: {}", charge.paid);
    println!("  Captured: {}", charge.captured);

    if let Some(card) = &charge.card {
        println!("  Card: {} ending in {}", card.brand, card.last4);
    }

    if let Some(description) = &charge.description {
        println!("  Description: {}", description);
    }

    Ok(())
}
