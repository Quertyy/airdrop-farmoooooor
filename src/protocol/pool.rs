use std::str::FromStr;

use ethers::prelude::*;
use crate::runner::parser::JsonPool;

#[derive(Clone,Debug, PartialEq)]
pub struct Pool {
    pub name: String,
    pub address: Address,
    pub token_0: Address,
    pub token_1: Address,
}

#[derive(Clone,Debug, PartialEq)]
pub enum PoolVariant {
    UniswapV2,
    UniswapV3,
}


impl Pool {
    pub fn new(
        name: String,
        address: Address,
    ) -> Self {
        // TODO: Get token addresses from pool
        Self {
            name,
            address,
            token_0: Address::from_str("0x5AEa5775959fBC2557Cc8789bC1bf90A239D9a91").unwrap(),
            token_1: Address::from_str("0x3355df6D4c9C3035724Fd0e3914dE96A5a83aaf4").unwrap(),
        }
    }

    pub fn from_json(pool: &JsonPool) -> Pool {
        Pool {
            name: pool.name.clone(),
            address: Address::from_str(&pool.address).unwrap(),
            token_0: Address::from_str("0x5AEa5775959fBC2557Cc8789bC1bf90A239D9a91").unwrap(),
            token_1: Address::from_str("0x3355df6D4c9C3035724Fd0e3914dE96A5a83aaf4").unwrap(),
        }
    }
}