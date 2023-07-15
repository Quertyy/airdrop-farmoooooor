use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use ethers::prelude::*;

use crate::prelude::pool::{Pool, PoolVariant};
use crate::runner::parser::JsonDex;

#[derive(Clone,Debug, PartialEq)]
pub struct Dex {
    pub name: String,
    pub router: Address,
    pub factory: Address,
    pub chain: Chain,
    pub pool_variant: PoolVariant,
    pub pools: Vec<Pool>,
}

impl Dex {
    pub fn new(
        name: String, 
        router: Address, 
        factory: Address, 
        chain: Chain,
        pool_variant: PoolVariant,
        pools: Vec<Pool>
    ) -> Self {
        Self {
            name,
            router,
            factory,
            chain,
            pool_variant,
            pools,
        }
    }

    pub fn from_json(chain: Chain, dex: &JsonDex) -> Dex {
        let pool_variant = match dex.variant.as_str().to_lowercase().as_str() {
            "uniswapv2" => PoolVariant::UniswapV2,
            "uniswapv3" => PoolVariant::UniswapV3,
            _ => PoolVariant::UniswapV2,
        };

        Dex {
            name: dex.name.clone(),
            factory: Address::from_str(&dex.factory).unwrap(),
            router: Address::from_str(&dex.router).unwrap(),
            chain,
            pool_variant,
            pools: dex.pools.iter().map(|pool| Pool::from_json(pool)).collect(),
        }
    }
}

impl Display for Dex {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.name)
    }
}