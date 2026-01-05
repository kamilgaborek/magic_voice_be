use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

/// Status lifecycle for an Order:
/// - PENDING: Order created, awaiting processing
/// - PROCESSING: Order is being processed
/// - DONE: Order completed successfully
/// - FAILED: Order processing failed
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum OrderStatus {
    Pending,
    Processing,
    Done,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: i64,
    pub status: OrderStatus,
    pub input_data: Value,
    pub result_video_url: Option<String>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

