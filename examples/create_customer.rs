//! Example: Creating a customer with a card
//!
//! **IMPORTANT SECURITY NOTE:**
//! This example creates tokens with raw card data for testing purposes only.
//! If you receive an "unsafe_credit_card_param" error, enable "Allow unsafe card parameters"
//! in your PAY.JP dashboard Test mode settings: https://pay.jp/d/settings
//!
//! In production, always use PAY.JP.js to create tokens client-side.
//!
//! Run with: cargo run --example create_customer

use payjp::{CardDetails, CreateCustomerParams, CreateTokenParams, PayjpClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("PAYJP_SECRET_KEY")
        .expect("PAYJP_SECRET_KEY environment variable not set");

    let client = PayjpClient::new(api_key)?;

    // Create a token for the card
    let card = CardDetails::new("4242424242424242", 12, 2030, "123")
        .name("Hanako Tanaka")
        .email("tanaka@example.com");

    println!("Creating token...");
    let token = client
        .tokens()
        .create(CreateTokenParams::from_card(card))
        .await?;

    println!("Token created: {}", token.id);

    // Create a customer with the card
    println!("\nCreating customer...");
    let customer = client
        .customers()
        .create(
            CreateCustomerParams::new()
                .email("tanaka@example.com")
                .description("VIP Customer")
                .card(token.id)
                .metadata("customer_type", "vip")
                .metadata("signup_date", "2024-01-01"),
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
