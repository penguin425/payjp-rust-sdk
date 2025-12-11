//! Term resource and service implementation.

use crate::client::PayjpClient;
use crate::error::PayjpResult;
use crate::params::ListParams;
use crate::response::ListResponse;
use serde::{Deserialize, Serialize};

/// A term represents an aggregation period for transactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
    /// Unique identifier for the term (prefixed with `tm_`).
    pub id: String,

    /// Object type (always "term").
    pub object: String,

    /// Whether this term was created in live mode.
    pub livemode: bool,

    /// Start date of the term (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,

    /// End date of the term (Unix timestamp, optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_at: Option<i64>,

    /// Charge count during this term.
    pub charge_count: i64,

    /// Refund count during this term.
    pub refund_count: i64,

    /// Dispute count during this term (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_count: Option<i64>,
}

/// Service for retrieving terms.
pub struct TermService<'a> {
    client: &'a PayjpClient,
}

impl<'a> TermService<'a> {
    /// Create a new term service.
    pub(crate) fn new(client: &'a PayjpClient) -> Self {
        Self { client }
    }

    /// Retrieve a term by ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::PayjpClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let term = client.terms().retrieve("tm_xxxxx").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn retrieve(&self, term_id: &str) -> PayjpResult<Term> {
        let path = format!("/terms/{}", term_id);
        self.client.get(&path).await
    }

    /// List all terms.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use payjp::{PayjpClient, ListParams};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PayjpClient::new("sk_test_xxxxx");
    /// let terms = client.terms().list(
    ///     ListParams::new().limit(10)
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, params: ListParams) -> PayjpResult<ListResponse<Term>> {
        self.client.get_with_params("/terms", &params).await
    }
}
