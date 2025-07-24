use axum::{
    routing::{get, post},
    Json, Router
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::str::FromStr;
use alloy_primitives::{keccak256, B256, U256};
use axum::extract::Query;
use crate::config::Config;
use crate::models::{HistoricalSwap, HistoryQuery, PairQuery, PairResponse, PriceResponse, TradingPair};
use crate::routes::history::get_historical_event;
use crate::routes::price::{PriceRequest, TradeRequest};
use crate::services::DexAggregator;

mod config;
mod models;
mod routes;
mod services;


// Handler for a simple health check
async fn health() -> &'static str {
    "DEX Aggregator backend is running"
}

// Handler for price queries (dummy implementation)
#[axum::debug_handler]
async fn get_price(Json(req): Json<PriceRequest>) -> Json<PriceResponse> {
    let aggr: DexAggregator  = DexAggregator::new(&Config::default());
    let price = aggr.get_best_price(&req.input_token, &req.output_token, req.amount).await.unwrap();
    Json(PriceResponse {
        input_amount: req.amount,
        output_amount: price[0].output_amount,
        dex: price[0].dex.clone(),
        best_price: price[0].best_price,
    })
}

#[axum::debug_handler]
async fn get_history(Query(params): Query<HistoryQuery>) -> Json<Vec<HistoricalSwap>> {
    // TODO Change rpc_url with your own value
    let rpc_url = "";
    get_historical_event(params.pair.clone(), rpc_url, B256::from_str(params.event_sig.as_str()).unwrap()).await
}

async fn get_uniswap_pair(Query(params): Query<PairQuery>) -> Json<PairResponse> {
    let aggr: DexAggregator  = DexAggregator::new(&Config::default());
    let pair_addr = aggr.get_uni_pair(&params.from_token, &params.to_token).await;
    return Json(pair_addr)
}

async fn get_sushiswap_pair(Query(params): Query<PairQuery>) -> Json<PairResponse> {
    let aggr: DexAggregator  = DexAggregator::new(&Config::default());
    let pair_addr = aggr.get_sushi_pair(&params.from_token, &params.to_token).await;
    return Json(pair_addr)
}

async fn trade_execution(Json(req): Json<TradeRequest>) {
    let aggr: DexAggregator  = DexAggregator::new(&Config::default());
    aggr.swap(&req.pair, &req.input_token, &req.output_token, &req.receiver, req.amount, req.slippage, req.time);
}

#[tokio::main]
async fn main() {
    // Build our application with routes
    let app = Router::new()
        .route("/", get(health))
        .route("/price", post(get_price))
        .route("/get-uni-pair", get(get_uniswap_pair))
        .route("/get-sushi-pair", get(get_sushiswap_pair))
        .route("/history", get(get_history))
        .route("/trade", post(trade_execution));
    // Set the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(&addr).await.unwrap(),
        app,
    ).await.unwrap();
}


#[tokio::test]
async fn test_main() {
    let client = reqwest::Client::new();
    let request = PriceRequest{
        input_token: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
        output_token: "0xdAC17F958D2ee523a2206206994597C13D831ec7".to_string(),
        amount: U256::from(2e18),
    };
    let response = client.post("http://127.0.0.1:3000/price").json(&request).send().await.unwrap();
    assert!(response.status() == reqwest::StatusCode::OK);
    let result = response.json::<PriceResponse>().await.unwrap();
    println!("best price is {}", result.best_price);


    let pair_addr = client.get("http://127.0.0.1:3000/get-sushi-pair").query(&[("from_token", "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), ("to_token", "0xdAC17F958D2ee523a2206206994597C13D831ec7")]).send().await.unwrap();
    let mut pair = pair_addr.json::<PairResponse>().await.unwrap();
    println!("pair address is: {:?}", pair);


    let pair_addr = client.get("http://127.0.0.1:3000/get-uni-pair").query(&[("from_token", "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"), ("to_token", "0xdAC17F958D2ee523a2206206994597C13D831ec7")]).send().await.unwrap();
    pair = pair_addr.json::<PairResponse>().await.unwrap();
    println!("pair address is: {:?}", pair);



    let rpc_url = "https://rpc.ankr.com/eth";
    let swap_event = "Swap(address,uint256,uint256,uint256,uint256,address)";
    let swap_hash = keccak256(swap_event.as_bytes());
    let history_response = client.get("http://127.0.0.1:3000/history").query(&[("pair", pair.pair_address.to_string()), ("event_sig", swap_hash.to_string())]).send().await.unwrap();
    // println!("history response is: {:?}", history_response);
    let history = history_response.json::<Vec<HistoricalSwap>>().await.unwrap();
    println!("history response is: {:?}", history);
}
