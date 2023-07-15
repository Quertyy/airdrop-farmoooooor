use ethers::prelude::{k256::ecdsa::SigningKey, *};

pub fn read_env_vars() -> Vec<(String, String)> {
    let mut env_vars = Vec::new();
    let keys = vec![
        "RPC_URL_WSS",
        "RPC_URL_HTTP",
        "PRIVATE_KEY",
    ];
    for key in keys {
        let value = dotenv::var(key).expect(&format!(
            "Required environment variable \"{}\" not set",
            key
        ));
        env_vars.push((key.to_string(), value));
    }
    env_vars
}

pub async fn get_ws_provider() -> Provider<Ws> {
    let env_var = "RPC_URL_WS";
    let url = 
        dotenv::var(&env_var).expect(format!("{} not found in .env file", env_var).as_str());
    Provider::<Ws>::connect(&url)
        .await
        .expect("Could not connect to RPC")
}

pub async fn get_http_provider() -> Provider<Http> {
    let env_var = "RPC_URL_HTTP";
    let url = 
        dotenv::var(&env_var).expect(format!("{} not found in .env file", env_var).as_str());
    Provider::<Http>::try_from(url)
        .expect("Could not connect to RPC")
}

pub async fn setup_signer(
    provider: Provider<Http>,
    user_wallet: LocalWallet
) -> SignerMiddleware<Provider<Http>, Wallet<SigningKey>> {
    let chain_id = provider
        .get_chainid()
        .await
        .expect("Failed to get chain id");
    let wallet = user_wallet.with_chain_id(chain_id.as_u64());

    SignerMiddleware::new(provider, wallet)
}