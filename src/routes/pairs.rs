use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;

use crate::{services::DexAggregator};
use crate::models::PairResponse;


pub async fn get_uni_pair(
    State(aggregator): State<Arc<DexAggregator>>,
    from_token: &str,
    to_token: &str
) -> Json<PairResponse> {
    let pair = aggregator.get_uni_pair(from_token, to_token).await;
    Json(pair)
}

pub async fn get_sushi_pair(
    State(aggregator): State<Arc<DexAggregator>>,
    from_token: &str,
    to_token: &str
) -> Json<PairResponse> {
    let pair = aggregator.get_sushi_pair(from_token, to_token).await;
    Json(pair)
}
