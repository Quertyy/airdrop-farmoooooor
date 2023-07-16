use std::fmt::{Display, Formatter, Result as FmtResult};
use ethers::prelude::*;
use crate::runner::console::Console;

//#[derive(Debug, Clone)]
//pub enum Choice {
//    Wallet(WalletChoice),
//    ChainType(Chain),
//    Type(TypeChoice),
//}

#[derive(Debug, Clone)]
pub enum Choice {
    Wallet,
    ChainType,
    Type,
}

#[derive(Debug, Clone)]
pub enum WalletChoice {
    Create,
    Import,
}

impl Display for WalletChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            WalletChoice::Create => write!(f, "Create a new wallet"),
            WalletChoice::Import => write!(f, "Import an existing address"),
        }
    }
}


#[derive(Debug, Clone)]
pub enum TypeChoice {
    Transfert,
    DeFi,
    NFT,
}

impl Display for TypeChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TypeChoice::Transfert => write!(f, "Transfert"),
            TypeChoice::DeFi => write!(f, "DeFi"),
            TypeChoice::NFT => write!(f, "NFT"),
        }
    }
}