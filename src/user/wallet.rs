use ethers::{prelude::{k256::ecdsa::SigningKey, *},
    core::rand,
    signers::{coins_bip39::English, MnemonicBuilder}
};
use crate::utils::dotenv::setup_signer;
use eyre::Result;

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
        let (address, nonce, signer) = Self::get_wallet(pkey, provider).await;
        Self {
            signer,
            address,
            nonce,
            bridged_amount: U256::zero(),
            transactions_value_amount: U256::zero(),
            erc20_tokens: vec![],
        }
    }

    pub async fn from_mnemonic(phrase: &str, provider: Provider<Http>) -> Result<Self> {
        let index = 0u32;
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .index(index)?
            .build()?;

        let (address, nonce, signer) = Self::get_wallet(wallet, provider).await;
        Ok(Self {
            signer,
            address,
            nonce,
            bridged_amount: U256::zero(),
            transactions_value_amount: U256::zero(),
            erc20_tokens: vec![],
        })
    }

    pub async fn generate_wallet(provider: Provider<Http>) -> Result<Self> {
        let mut rng = rand::thread_rng();
        let wallet = MnemonicBuilder::<English>::default()
            .word_count(24)
            .derivation_path("m/44'/60'/0'/2/1")?
            .build_random(&mut rng)?;
        
        let (address, nonce, signer) = Self::get_wallet(wallet, provider).await;
        Ok(Self {
            signer,
            address,
            nonce,
            bridged_amount: U256::zero(),
            transactions_value_amount: U256::zero(),
            erc20_tokens: vec![],
        })
    }

    async fn get_wallet(
        wallet: LocalWallet, 
        provider: Provider<Http>
    ) -> (Address, U256, SignerMiddleware<Provider<Http>, Wallet<SigningKey>>) {
        let signer = setup_signer(provider.clone(), wallet).await;
        let address = signer.address();
        let nonce = provider.get_transaction_count(address, None).await.unwrap();
        (address, nonce, signer)
    }
}