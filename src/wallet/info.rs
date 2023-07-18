use ethers::{prelude::{k256::ecdsa::SigningKey, *},
    core::rand,
    utils::to_checksum,
    signers::{coins_bip39::English, MnemonicBuilder}
};

use crate::prelude::WalletSigner;
use eyre::Result;


#[derive(Debug)]
pub struct WalletInfo {
    pub wallet_signer: WalletSigner,
    pub bridged_amount: U256,
    pub transactions_value_amount: U256,
    pub erc20_tokens: Vec<Address>,
}

impl WalletInfo {
    
}