//! Example: 3D Secure authentication
//!
//! This example demonstrates 3D Secure authentication for a customer's card.
//! Note: 3D Secure requests require a card ID (car_xxxxx), not a token ID.
//!
//! To get a token for testing, run:
//!   PAYJP_PUBLIC_KEY=pk_test_xxxxx PAYJP_PUBLIC_PASSWORD=password cargo run --example create_token_public
//!
//! Run with:
//!   PAYJP_SECRET_KEY=sk_test_xxxxx PAYJP_TOKEN_ID=tok_xxxxx cargo run --example three_d_secure

use payjp::{CardOrId, CreateCustomerParams, CreateThreeDSecureRequestParams, PayjpClient};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("PAYJP_SECRET_KEY")
        .expect("PAYJP_SECRET_KEY environment variable not set");

    let token_id = env::var("PAYJP_TOKEN_ID")
        .expect("PAYJP_TOKEN_ID environment variable not set. Run create_token_public first to get a token.");

    let client = PayjpClient::new(api_key)?;

    // Step 1: Create a customer with the token to get a card ID
    println!("Creating customer with token: {}", token_id);
    let customer = client
        .customers()
        .create(
            CreateCustomerParams::new()
                .email("tds-test@example.com")
                .description("3D Secure Test Customer")
                .card(token_id),
        )
        .await?;

    println!("✓ Customer created: {}", customer.id);

    // Step 2: Extract the card ID from the customer
    let card_id = match &customer.default_card {
        Some(CardOrId::Id(id)) => id.clone(),
        Some(CardOrId::Card(card)) => card.id.clone(),
        None => {
            return Err("Customer has no default card".into());
        }
    };

    println!("  Card ID: {}", card_id);

    // Step 3: Create a 3D Secure request for the card
    println!("\nCreating 3D Secure request...");
    let tds_request = client
        .three_d_secure_requests()
        .create(CreateThreeDSecureRequestParams::new(&card_id))
        .await?;

    println!("✓ 3D Secure request created!");
    println!("  ID: {}", tds_request.id);

    if let Some(resource_id) = &tds_request.resource_id {
        println!("  Card ID: {}", resource_id);
    }

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

    println!("✓ Retrieved request: {}", retrieved.id);

    // Note: In a real application, after the user completes 3D Secure authentication,
    // you would retrieve the request again to check its status, then use the card
    // for charges or subscriptions.

    // Clean up: delete the test customer
    println!("\n--- Cleaning up ---");
    client.customers().delete(&customer.id).await?;
    println!("✓ Customer deleted");

    Ok(())
}
