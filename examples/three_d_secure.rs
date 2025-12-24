//! Example: 3D Secure authentication
//!
//! This example demonstrates 3D Secure authentication using a pre-created token.
//!
//! To get a token for testing, run:
//!   PAYJP_PUBLIC_KEY=pk_test_xxxxx PAYJP_PUBLIC_PASSWORD=password cargo run --example create_token_public
//!
//! Run with:
//!   PAYJP_SECRET_KEY=sk_test_xxxxx PAYJP_TOKEN_ID=tok_xxxxx cargo run --example three_d_secure

use payjp::{CreateThreeDSecureRequestParams, PayjpClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("PAYJP_SECRET_KEY")
        .expect("PAYJP_SECRET_KEY environment variable not set");

    let token_id = env::var("PAYJP_TOKEN_ID")
        .expect("PAYJP_TOKEN_ID environment variable not set. Run create_token_public first to get a token.");

    let client = PayjpClient::new(api_key)?;

    println!("Using token: {}", token_id);

    // Create a 3D Secure request for the token
    println!("\nCreating 3D Secure request...");
    let tds_request = client
        .three_d_secure_requests()
        .create(
            CreateThreeDSecureRequestParams::new("token", &token_id)
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
