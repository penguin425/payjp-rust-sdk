//! Event resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::ListParams;
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An event represents a notification about changes to resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Unique identifier for the event (prefixed with `evnt_`).
    pub id: String,

    /// Object type (always "event").
    pub object: String,

    /// Whether this event was created in live mode.
    pub livemode: bool,

    /// Event creation timestamp (Unix timestamp).
    pub created: i64,

    /// Type of event.
    #[serde(rename = "type")]
    pub event_type: EventType,

    /// Data associated with the event.
    pub data: EventData,

    /// Number of pending webhooks for this event (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_webhooks: Option<i64>,
}

/// Type of event that occurred.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// Charge was created.
    #[serde(rename = "charge.created")]
    ChargeCreated,

    /// Charge was updated.
    #[serde(rename = "charge.updated")]
    ChargeUpdated,

    /// Charge succeeded.
    #[serde(rename = "charge.succeeded")]
    ChargeSucceeded,

    /// Charge failed.
    #[serde(rename = "charge.failed")]
    ChargeFailed,

    /// Charge was captured.
    #[serde(rename = "charge.captured")]
    ChargeCaptured,

    /// Charge was refunded.
    #[serde(rename = "charge.refunded")]
    ChargeRefunded,

    /// Customer was created.
    #[serde(rename = "customer.created")]
    CustomerCreated,

    /// Customer was updated.
    #[serde(rename = "customer.updated")]
    CustomerUpdated,

    /// Customer was deleted.
    #[serde(rename = "customer.deleted")]
    CustomerDeleted,

    /// Card was created.
    #[serde(rename = "customer.card.created")]
    CustomerCardCreated,

    /// Card was updated.
    #[serde(rename = "customer.card.updated")]
    CustomerCardUpdated,

    /// Card was deleted.
    #[serde(rename = "customer.card.deleted")]
    CustomerCardDeleted,

    /// Plan was created.
    #[serde(rename = "plan.created")]
    PlanCreated,

    /// Plan was updated.
    #[serde(rename = "plan.updated")]
    PlanUpdated,

    /// Plan was deleted.
    #[serde(rename = "plan.deleted")]
    PlanDeleted,

    /// Subscription was created.
    #[serde(rename = "subscription.created")]
    SubscriptionCreated,

    /// Subscription was updated.
    #[serde(rename = "subscription.updated")]
    SubscriptionUpdated,

    /// Subscription was deleted.
    #[serde(rename = "subscription.deleted")]
    SubscriptionDeleted,

    /// Subscription was paused.
    #[serde(rename = "subscription.paused")]
    SubscriptionPaused,

    /// Subscription was resumed.
    #[serde(rename = "subscription.resumed")]
    SubscriptionResumed,

    /// Subscription was canceled.
    #[serde(rename = "subscription.canceled")]
    SubscriptionCanceled,

    /// Subscription renewal succeeded.
    #[serde(rename = "subscription.renewed")]
    SubscriptionRenewed,

    /// Transfer was created.
    #[serde(rename = "transfer.created")]
    TransferCreated,

    /// Other event types not explicitly handled.
    #[serde(other)]
    Other,
}

/// Event data containing the affected resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    /// The previous attributes of the resource (for update events, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<Value>,

    /// The resource object affected by the event.
    pub object: Value,
}

/// Service for retrieving events.
pub struct EventService<'a> {
    client: &'a PayjpClient,
}

impl<'a> EventService<'a> {
    /// Create a new event service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Retrieve an event by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let event = client.events().retrieve("evnt_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, event_id: &str) -> PayjpResult<Event> {
        let path = format!("/events/{}", event_id);
        self.client.get(&path).await
    }

    /// List all events.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let events = client.events().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<Event>> {
        self.client.get_with_params("/events", &params).await
    }
}
