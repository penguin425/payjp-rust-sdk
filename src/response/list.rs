//! List response types for paginated API endpoints.

use serde::{Deserialize, Serialize};

/// A paginated list response from PAY.JP API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResponse<T> {
    /// The object type (always "list").
    pub object: String,

    /// The list of items.
    pub data: Vec<T>,

    /// Whether there are more items available.
    pub has_more: bool,

    /// The URL for this list endpoint.
    pub url: String,

    /// Total count of items.
    pub count: i64,
}

impl<T> Default for ListResponse<T> {
    fn default() -> Self {
        Self {
            object: "list".to_string(),
            data: Vec::new(),
            has_more: false,
            url: String::new(),
            count: 0,
        }
    }
}
