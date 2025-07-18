use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingPair {
    pub pair_address: String,
    pub token0: String,
    pub token1: String,
    pub dex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_24h: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct PairQuery {
    pub from_token: String,
    pub to_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PairResponse {
    pub pair_address: String,
}

impl TradingPair {
    pub fn new(address: &str, token_a: &str, token_b: &str, dex: &str) -> Self {
        let (token0, token1) = sort_tokens(token_a, token_b);
        Self {
            pair_address: address.to_string(),
            token0,
            token1,
            dex: dex.to_string(),
            volume_24h: None,
        }
    }
}

fn sort_tokens(a: &str, b: &str) -> (String, String) {
    if a < b {
        (a.to_string(), b.to_string())
    } else {
        (b.to_string(), a.to_string())
    }
}
