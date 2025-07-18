use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub cache_ttl_sec: u64,
    pub rate_limit_quota: u32,
    pub uniswap_endpoint: String,
    pub uniswap_rpc_url: String,
    pub uniswap_router: String,
    pub sushiswap_endpoint: String,
    pub sushiswap_rpc_url: String,
    pub sushiswap_router: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache_ttl_sec: 30,
            rate_limit_quota: 100,
            uniswap_endpoint: "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v2".into(),
            uniswap_rpc_url: "https://rpc.ankr.com/eth".into(),
            uniswap_router: "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".into(),
            sushiswap_endpoint: "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v2".into(),
            sushiswap_rpc_url: "https://rpc.ankr.com/eth".into(),
            sushiswap_router: "0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F".into()
        }
    }
}
