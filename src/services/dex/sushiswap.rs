use std::str::FromStr;
use alloy_core::primitives::U256;
use alloy_sol_types::sol;
use alloy_primitives::{Address};
use crate::models::{PriceResponse};
use reqwest::{Client};

pub struct SushiswapFetcher {
    client: Client,
    endpoint: String,
    rpc_url: String,
    router: String
}

sol! {
    #[sol(rpc)]
    interface IERC20 {
        function balanceOf(address _account) external view returns (uint256);
        function decimals() external view returns (uint256);
    }

    #[sol(rpc)]
    interface ISushiswapV2Router02 {
        function getAmountsOut(uint256 amountIn, address[] memory path) external view returns (uint256[] memory amounts);
        function swapExactTokensForTokens(uint256 amountIn, uint256 amountOutMin, address[] memory path, address to, uint256 deadline) external returns (uint256[] memory amounts);
    }

    #[sol(rpc)]
    interface ISushiswapV2Pair {
        function token0() external view returns (address);
        function token1() external view returns (address);
        function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
    }

    #[sol(rpc)]
    interface ISushiswapV2Factory {
        function createPair(address token0, address token1) external returns (address pair);
        function getPair(address token0, address token1) external view returns (address pair);
    }
}

impl SushiswapFetcher {
    pub fn new(endpoint: &str, rpc_url: &str, router: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.to_string(),
            rpc_url: rpc_url.to_string(),
            router: router.to_string()
        }
    }

    pub async fn get_price_from_router(&self, from_token: &str, to_token: &str, amount: U256) -> Result<PriceResponse, Box<dyn std::error::Error + Send + Sync>> {
        // TODO Implement logic to get sushiswap price
        Ok(PriceResponse {
            input_amount: amount,
            output_amount: U256::from(0),
            best_price: U256::from(0),
            dex: "Sushiswap".to_string()
        })
    }

    pub async fn swap(&self, from_token: &str, to_token: &str, receiver: &str, amount: U256, slippage: U256, time: U256) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO Implement logic to execute swap
        Ok(())
    }

    pub async fn get_pair(&self, from_token: &str, to_token: &str) -> Result<Address, Box<dyn std::error::Error + Send + Sync>> {
        // TODO Implement to get sushiswap pair given token addresses
        Ok(Address::from_str("0x0000000000000000000000000000000000000000").unwrap())
    }
}