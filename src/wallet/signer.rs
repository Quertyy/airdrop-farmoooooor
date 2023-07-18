use std::{
    io::{self, Write},
    fs,
};
use ethers::{prelude::{k256::ecdsa::SigningKey, *},
    core::rand,
    utils::to_checksum,
    signers::{coins_bip39::English, MnemonicBuilder}
};
use eyre::Result;
use crate::utils::dotenv::setup_signer;

#[derive(Debug)]
pub struct WalletSigner {
    pub signer: SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
    pub address: Address,
    pub nonce: U256,
}

impl WalletSigner {
    pub async fn from_pkey(pkey: LocalWallet, provider: &Provider<Http>) -> Self {
        let (address, nonce, signer) = Self::get_wallet(pkey, provider).await;
        Self {
            signer,
            address,
            nonce,
        }
    }

    pub async fn generate_wallets(provider: &Provider<Http>, mut quantity: u8) -> Vec<Self> {
        if quantity > 10_u8 {
            quantity = 10;
        }
        let mut wallets = vec![];
        for _ in 0..quantity {
            let wallet = Self::generate_wallet(provider).await.unwrap();
            wallets.push(wallet);
        }
        println!("Please save it in a safe place!");
        wallets
    }

    pub async fn from_mnemonic(provider: &Provider<Http>) -> Result<Self> {
        let mut input = String::new();
        print!("Mnemonic : ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Error reading line");

        let phrase: &str = &input.trim();
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
        })
    }

    async fn generate_wallet(provider: &Provider<Http>) -> Result<Self> {
        let dir = tempfile::tempdir()?;
        let mut rng = rand::thread_rng();
        let wallet = MnemonicBuilder::<English>::default()
            .word_count(24)
            .derivation_path("m/44'/60'/0'/2/1")?
            .write_to(dir.as_ref())
            .build_random(&mut rng)?;

        let phrase_path = dir.as_ref().join(to_checksum(&wallet.address(), None));
        let content = fs::read_to_string(phrase_path)?;
        let phrase = content.trim().to_string();
        println!("Your passphrase: {}", phrase);

        let (address, nonce, signer) = Self::get_wallet(wallet, provider).await;
        Ok(Self {
            signer,
            address,
            nonce,
        })
    }

    async fn get_wallet(
        wallet: LocalWallet, 
        provider: &Provider<Http>
    ) -> (Address, U256, SignerMiddleware<Provider<Http>, Wallet<SigningKey>>) {
        let signer = setup_signer(provider.clone(), wallet).await;
        let address = signer.address();
        let nonce = provider.get_transaction_count(address, None).await.unwrap();
        (address, nonce, signer)
    }
}



