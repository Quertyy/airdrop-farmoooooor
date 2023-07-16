use ethers::prelude::{k256::ecdsa::SigningKey, *};
use crate::utils::dotenv::setup_signer;

pub struct WalletInfo {
    pub signer: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    pub address: Address,
    pub nonce: U256,
    pub bridged_amount: U256,
    pub transactions_value_amount: U256,
    pub erc20_tokens: Vec<Address>,
}

impl WalletInfo {
    pub async fn from_pkey(pkey: LocalWallet, provider: Provider<Http>) -> Self {
        let address = pkey.address();
        let signer = setup_signer(provider.clone(), pkey).await;
        let nonce = provider.get_transaction_count(address, None).await.unwrap();
        Self {
            signer,
            address,
            nonce,
            bridged_amount: U256::zero(),
            transactions_value_amount: U256::zero(),
            erc20_tokens: vec![],
        }
    }
}