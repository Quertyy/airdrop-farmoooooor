use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use crate::runner::{get_all_chains, get_all_dexes};
use crate::user::WalletInfo;
use crate::runner::{WalletChoice, Console};
use crate::utils::dotenv::get_all_wallets;
use ethers::prelude::*;
use eyre::Result;

pub fn run() -> Result<()> {
    let mut console = Console::new();
    console.display(vec![WalletChoice::Create, WalletChoice::Import], "Wallet")?;

    Ok(())
}