//! Example: 3D Secure authentication
//!
//! **IMPORTANT SECURITY NOTE:**
//! This example creates tokens with raw card data for testing purposes only.
//! If you receive an "unsafe_credit_card_param" error, enable "Allow unsafe card parameters"
//! in your PAY.JP dashboard Test mode settings: https://pay.jp/d/settings
//!
//! In production, always use PAY.JP.js to create tokens client-side.
//!
//! Run with: cargo run --example three_d_secure

use payjp::{CardDetails, CreateThreeDSecureRequestParams, CreateTokenParams, PayjpClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("PAYJP_SECRET_KEY")
        .expect("PAYJP_SECRET_KEY environment variable not set");

    let client = PayjpClient::new(api_key)?;

    // Create a token
    let card = CardDetails::new("4242424242424242", 12, 2030, "123")
        .name("Ichiro Suzuki")
        .email("suzuki@example.com");

    println!("Creating token...");
    let token = client
        .tokens()
        .create(CreateTokenParams::from_card(card))
        .await?;

    println!("✓ Token created: {}", token.id);

    // Create a 3D Secure request for the token
    println!("\nCreating 3D Secure request...");
    let tds_request = client
        .three_d_secure_requests()
        .create(
            CreateThreeDSecureRequestParams::new("token", &token.id)
                .return_url("https://example.com/callback")
                .state("custom_state_data"),
        )
        .await?;

    println!("✓ 3D Secure request created!");
    println!("  ID: {}", tds_request.id);
    println!("  Status: {:?}", tds_request.status);

    if let Some(auth_url) = &tds_request.authentication_url {
        println!("  Authentication URL: {}", auth_url);
        println!("\n  → User should complete authentication at this URL");
    }

    // Retrieve the request to check status
    println!("\nRetrieving 3D Secure request...");
    let retrieved = client
        .three_d_secure_requests()
        .retrieve(&tds_request.id)
        .await?;

    println!("✓ Status: {:?}", retrieved.status);

    // Note: In a real application, after the user completes 3D Secure authentication,
    // you would call tds_finish on the token or charge to complete the process:
    //
    // let completed_token = client.tokens().tds_finish(&token.id).await?;
    // or
    // let completed_charge = client.charges().tds_finish(&charge.id).await?;

    Ok(())
}
