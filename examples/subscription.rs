//! Example: Creating a subscription
//!
//! **IMPORTANT SECURITY NOTE:**
//! This example creates tokens with raw card data for testing purposes only.
//! If you receive an "unsafe_credit_card_param" error, enable "Allow unsafe card parameters"
//! in your PAY.JP dashboard Test mode settings: https://pay.jp/d/settings
//!
//! In production, always use PAY.JP.js to create tokens client-side.
//!
//! Run with: cargo run --example subscription

use payjp::{
    CardDetails, CreateCustomerParams, CreatePlanParams, CreateSubscriptionParams,
    CreateTokenParams, PayjpClient, PlanInterval,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("PAYJP_SECRET_KEY")
        .expect("PAYJP_SECRET_KEY environment variable not set");

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

    // Step 2: Create a customer with a card
    let card = CardDetails::new("4242424242424242", 12, 2030, "123")
        .name("Jiro Sato")
        .email("sato@example.com");

    println!("\nCreating token...");
    let token = client
        .tokens()
        .create(CreateTokenParams::from_card(card))
        .await?;

    println!("Creating customer...");
    let customer = client
        .customers()
        .create(
            CreateCustomerParams::new()
                .email("sato@example.com")
                .card(token.id),
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
