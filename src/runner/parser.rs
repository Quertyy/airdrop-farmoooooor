use std::fs;
use std::str::FromStr;

#[allow(unused_imports)]
use ethers::types::{Chain, Address};
use serde::Deserialize;

use crate::prelude::dex::Dex;

#[derive(Deserialize)]
struct FileData {
    chains: Vec<JsonChain>,
}

#[derive(Deserialize)]
pub struct JsonChain {
    pub name: String,
    pub id: String,
    pub tokens: Vec<Token>,
    pub dexes: Vec<JsonDex>,
}

#[derive(Deserialize)]
pub struct Token {
    pub symbol: String,
    pub address: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JsonDex {
    pub name: String,
    pub factory: String,
    pub router: String,
    pub variant: String,
    pub pools: Vec<JsonPool>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct JsonPool {
    pub name: String,
    pub address: String,
}

pub fn get_all_chains() -> Vec<Chain> {
    let file_data: FileData = serde_json::from_str(&fs::read_to_string("src/contracts/dexs.json").unwrap()).unwrap();
    file_data.chains.iter().map(|chain| Chain::from_str(&chain.name.to_lowercase()).unwrap()).collect()
}

pub fn get_all_dexes(chain: &str) -> Option<Vec<Dex>> {
    let file_data: FileData = serde_json::from_str(&fs::read_to_string("src/contracts/dexs.json").unwrap()).unwrap();
    for chain_data in file_data.chains {
        if chain_data.name.to_lowercase() == chain.to_lowercase() {
            let dex_vec = chain_data.dexes.into_iter()
                .map(|dex| Dex::from_json(Chain::from_str(&chain_data.id.to_lowercase()).unwrap(), &dex))
                .collect();
            return Some(dex_vec);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{PoolVariant, Pool};

    #[test]
    fn test_get_dexs() {
        let dexes = get_all_dexes("zksync").unwrap();
        let zksync_uniswap_v2 = Dex {
            name: "SyncSwap".to_string(),
            router: Address::from_str("0x2da10A1e27bF85cEdD8FFb1AbBe97e53391C0295").unwrap(),
            factory: Address::from_str("0xf2DAd89f2788a8CD54625C60b55cD3d2D0ACa7Cb").unwrap(),
            chain: Chain::ZkSync,
            pool_variant: PoolVariant::UniswapV2,
            pools: vec![
                Pool {
                    name: "weth/usdc".to_string(),
                    address: Address::from_str("0x80115c708E12eDd42E504c1cD52Aea96C547c05c").unwrap(),
                    token_0: Address::from_str("0x5AEa5775959fBC2557Cc8789bC1bf90A239D9a91").unwrap(),
                    token_1: Address::from_str("0x3355df6D4c9C3035724Fd0e3914dE96A5a83aaf4").unwrap(),
                }
            ],
        };
        assert_eq!(dexes[1].pools[0], zksync_uniswap_v2.pools[0]);
    }
}