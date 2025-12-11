//! Example: Creating a charge
//!
//! Run with: cargo run --example create_charge

use payjp::{CardDetails, CreateChargeParams, CreateTokenParams, PayjpClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("PAYJP_SECRET_KEY")
        .expect("PAYJP_SECRET_KEY environment variable not set");

    let client = PayjpClient::new(api_key);

    // First, create a token with test card data
    // In production, tokens should be created client-side using PAY.JP.js
    let card = CardDetails::new(
        "4242424242424242", // Test card number
        12,                 // Expiration month
        2028,               // Expiration year
        "123",              // CVC
    )
    .name("山田太郎")
    .email("test@example.com");

    println!("Creating token...");
    let token = client
        .tokens()
        .create(CreateTokenParams::from_card(card))
        .await?;

    println!("Token created: {}", token.id);

    // Create a charge using the token
    println!("\nCreating charge...");
    let charge = client
        .charges()
        .create(
            CreateChargeParams::new(1000, "jpy")
                .card(token.id)
                .description("テスト支払い")
                .metadata("order_id", "12345")
                .metadata("customer_name", "山田太郎"),
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
