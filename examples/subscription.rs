//! Example: Creating a subscription
//!
//! This example demonstrates creating a subscription using a pre-created token.
//!
//! To get a token for testing, run:
//!   PAYJP_PUBLIC_KEY=pk_test_xxxxx PAYJP_PUBLIC_PASSWORD=password cargo run --example create_token_public
//!
//! Run with:
//!   PAYJP_SECRET_KEY=sk_test_xxxxx PAYJP_TOKEN_ID=tok_xxxxx cargo run --example subscription

use payjp::{
    CreateCustomerParams, CreatePlanParams, CreateSubscriptionParams,
    PayjpClient, PlanInterval,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("PAYJP_SECRET_KEY")
        .expect("PAYJP_SECRET_KEY environment variable not set");

    let token_id = env::var("PAYJP_TOKEN_ID")
        .expect("PAYJP_TOKEN_ID environment variable not set. Run create_token_public first to get a token.");

    let client = PayjpClient::new(api_key)?;

    // Step 1: Create a plan
    println!("Creating plan...");
    let plan = client
        .plans()
        .create(
            CreatePlanParams::new(980, "jpy", PlanInterval::Month)
                .name("Monthly Plan")
                .trial_days(7),
        )
        .await?;

    println!("✓ Plan created: {}", plan.id);
    println!("  Amount: ¥{}/month", plan.amount);
    if let Some(trial_days) = plan.trial_days {
        println!("  Trial days: {}", trial_days);
    }

    // Step 2: Create a customer with the token
    println!("\nCreating customer with token: {}", token_id);
    let customer = client
        .customers()
        .create(
            CreateCustomerParams::new()
                .email("sato@example.com")
                .card(token_id),
        )
        .await?;

    println!("✓ Customer created: {}", customer.id);

    // Step 3: Create a subscription
    println!("\nCreating subscription...");
    let subscription = client
        .subscriptions()
        .create(
            CreateSubscriptionParams::new(&customer.id, &plan.id),
        )
        .await?;

    println!("✓ Subscription created successfully!");
    println!("  ID: {}", subscription.id);
    println!("  Status: {:?}", subscription.status);
    println!("  Plan: {}", subscription.plan.id);

    if let Some(trial_end) = subscription.trial_end {
        println!("  Trial ends: {}", trial_end);
    }

    println!("\n--- Pausing subscription ---");
    let paused = client
        .subscriptions()
        .pause(&subscription.id, Default::default())
        .await?;

    println!("✓ Subscription paused: {:?}", paused.status);

    println!("\n--- Resuming subscription ---");
    let resumed = client
        .subscriptions()
        .resume(&subscription.id, Default::default())
        .await?;

    println!("✓ Subscription resumed: {:?}", resumed.status);

    // Clean up
    println!("\n--- Cleaning up ---");
    client.subscriptions().delete(&subscription.id).await?;
    println!("✓ Subscription deleted");

    client.plans().delete(&plan.id).await?;
    println!("✓ Plan deleted");

    client.customers().delete(&customer.id).await?;
    println!("✓ Customer deleted");

    Ok(())
}
