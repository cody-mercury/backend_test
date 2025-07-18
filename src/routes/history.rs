use std::str::FromStr;
use axum::{
    Json,
};
use alloy::sol_types::{SolEvent};
use serde::Deserialize;
use alloy_sol_types::sol;
use alloy_primitives::{B256};
use crate::{models::HistoricalSwap};

sol! {
     event Swap(
        address indexed sender,
        uint amount0In,
        uint amount1In,
        uint amount0Out,
        uint amount1Out,
        address indexed to
    );
}
#[derive(Deserialize)]
pub struct HistoryRequest {
    pair_address: String,
    limit: Option<u32>,
}

pub async fn get_historical_event(
    pair: String,
    rpc_url: &str,
    event_sig: B256,
) -> Json<Vec<HistoricalSwap>> {
    // TODO implement logic to fetch historical events from the DEX
    let mut historical_swaps = Vec::new();
    Json(historical_swaps)
}
