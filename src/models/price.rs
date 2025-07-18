use alloy_primitives::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceResponse {
    pub input_amount: U256,
    pub output_amount: U256,
    pub best_price: U256,
    pub dex: String,
}
