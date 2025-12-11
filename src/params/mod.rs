//! Parameter types for PAY.JP API requests.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata type for arbitrary key-value pairs.
///
/// PAY.JP supports up to 20 keys, with each key up to 40 characters
/// and each value up to 500 characters.
pub type Metadata = HashMap<String, String>;

/// Common parameters for list endpoints with pagination.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ListParams {
    /// Maximum number of items to return (default: 10, max: 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// Offset for pagination (default: 0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// Return items created since this timestamp (Unix timestamp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,

    /// Return items created until this timestamp (Unix timestamp).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<i64>,
}

impl ListParams {
    /// Create a new `ListParams` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit for the number of items to return.
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination.
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the since timestamp filter.
    pub fn since(mut self, since: i64) -> Self {
        self.since = Some(since);
        self
    }

    /// Set the until timestamp filter.
    pub fn until(mut self, until: i64) -> Self {
        self.until = Some(until);
        self
    }
}
