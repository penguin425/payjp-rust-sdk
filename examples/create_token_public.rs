//! Example: Creating a token with a public key
//!
//! This example demonstrates the recommended way to create tokens using a public key.
//! This approach is safer than using a secret key because:
//! - Public keys can only create tokens, not perform other operations
//! - Tokens can be created client-side without exposing sensitive keys
//! - Follows PAY.JP security best practices
//!
//! Run with:
//!   PAYJP_PUBLIC_KEY=pk_test_xxxxx cargo run --example create_token_public

use payjp::{CardDetails, CreateTokenParams, PayjpPublicClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get public key from environment variable
    let public_key = env::var("PAYJP_PUBLIC_KEY")
        .expect("PAYJP_PUBLIC_KEY environment variable not set. Use a public key like pk_test_xxxxx");

    // Create a public client (for token creation only)
    let client = PayjpPublicClient::new(public_key)?;

    println!("Creating token with public key...");

    // Create a token with test card data
    let card = CardDetails::new("4242424242424242", 12, 2030, "123")
        .name("Taro Yamada")
        .email("yamada@example.com");

    let token = client
        .tokens()
        .create(CreateTokenParams::from_card(card))
        .await?;

    println!("✓ Token created successfully!");
    println!("  ID: {}", token.id);
    println!("  Used: {}", token.used);
    println!("  Livemode: {}", token.livemode);
    println!("  Card brand: {}", token.card.brand);
    println!("  Card last 4: {}", token.card.last4);

    println!("\nYou can now use this token with your secret key:");
    println!("  export PAYJP_SECRET_KEY=sk_test_xxxxx");
    println!("  export PAYJP_TOKEN_ID={}", token.id);
    println!("  cargo run --example charge_with_token");

    Ok(())
}
