mod dex;
mod cache;

use alloy_primitives::U256;
pub use cache::PriceCache;
pub use dex::{UniswapFetcher, SushiswapFetcher};
use crate::models::{PairResponse, PriceResponse};

pub struct DexAggregator {
    uniswap: UniswapFetcher,
    sushiswap: SushiswapFetcher,
    cache: PriceCache,
}

impl DexAggregator {
    pub fn new(config: &crate::config::Config) -> Self {
        Self {
            uniswap: UniswapFetcher::new(&config.uniswap_endpoint, &config.uniswap_rpc_url, &config.uniswap_router),
            sushiswap: SushiswapFetcher::new(&config.sushiswap_endpoint, &config.sushiswap_rpc_url, &config.sushiswap_router),
            cache: PriceCache::new(config.cache_ttl_sec),
        }
    }

    pub async fn get_uni_pair(&self, from_token: &str, to_token: &str) -> PairResponse {
        let pair_addr = self.uniswap
            .get_pair(from_token, to_token)
            .await
            .expect("Failed to get pair from router");
        PairResponse {
            pair_address: pair_addr.to_string(),
        }
    }

    pub async fn get_sushi_pair(&self, from_token: &str, to_token: &str) -> PairResponse {
        let pair_addr = self.sushiswap
            .get_pair(from_token, to_token)
            .await
            .expect("Failed to get pair from router");
        PairResponse {
            pair_address: pair_addr.to_string(),
        }
    }

    pub async fn swap(&self, pair: &str, from_token: &str, to_token: &str, receiver: &str, amount: U256, slippage: U256, time: U256) {
        // TODO Implement the swap logic here
    }

    pub async fn get_best_price(&self, from_token: &str, to_token: &str, amount: U256) -> Result<Vec<PriceResponse>, Box<dyn std::error::Error + Send + Sync>> {
        // TODD Implement to get best price among 2 pairs
        Ok(vec![
            PriceResponse{
                input_amount: amount,
                output_amount: U256::from(0),
                best_price: U256::from(0),
                dex: "Uniswap".to_string()
            },
            PriceResponse{
                input_amount: amount,
                output_amount: U256::from(0),
                best_price: U256::from(0),
                dex: "Sushiswap".to_string()
            },
        ])
    }
}