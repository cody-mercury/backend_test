use alloy_primitives::U256;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalSwap {
    pub timestamp: DateTime<Utc>,
    pub input0_amount: U256,
    pub input1_amount: U256,
    pub output0_amount: U256,
    pub output1_amount: U256,
    pub recipient: String
}

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    pub pair: String,
    pub event_sig: String,
}