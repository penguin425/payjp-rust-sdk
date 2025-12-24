//! Example: Creating a charge
//!
//! This example demonstrates creating a charge using a pre-created token.
//!
//! To get a token for testing, run:
//!   PAYJP_PUBLIC_KEY=pk_test_xxxxx PAYJP_PUBLIC_PASSWORD=password cargo run --example create_token_public
//!
//! Run with:
//!   PAYJP_SECRET_KEY=sk_test_xxxxx PAYJP_TOKEN_ID=tok_xxxxx cargo run --example create_charge

use payjp::{CreateChargeParams, PayjpClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("PAYJP_SECRET_KEY")
        .expect("PAYJP_SECRET_KEY environment variable not set");

    let token_id = env::var("PAYJP_TOKEN_ID")
        .expect("PAYJP_TOKEN_ID environment variable not set. Run create_token_public first to get a token.");

    let client = PayjpClient::new(api_key)?;

    println!("Creating charge with token: {}", token_id);

    // Create a charge using the token
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
