use serde::{Deserialize, Serialize};
use alloy_primitives::U256;

#[derive(Serialize, Deserialize)]
pub struct PriceRequest {
    pub input_token: String,
    pub output_token: String,
    pub amount: U256,
}

#[derive(Serialize, Deserialize)]
pub struct TradeRequest {
    pub pair: String,
    pub input_token: String,
    pub output_token: String,
    pub amount: U256,
    pub receiver: String,
    pub slippage: U256,
    pub time: U256,
}
