use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use crate::runner::{get_all_chains, get_all_dexes};
use std::io::Result;
use ethers::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub enum Action {
    Transfert,
    DeFi,
    NFT,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Action::Transfert => write!(f, "Transfert"),
            Action::DeFi => write!(f, "DeFi"),
            Action::NFT => write!(f, "NFT"),
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::DeFi
    }
}

#[derive(Debug)]
pub enum DefiAction {
    Swap,
}


pub fn display_chain_choice() -> Result<Chain> {
    let chains = get_all_chains();
    let chain_selection = Select::with_theme(&ColorfulTheme::default())
        .items(&chains)
        .with_prompt("Select a chain")
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap();

    match chain_selection {
        Some(index) => Ok(chains[index]),
        None => {
            println!("You have not selected anything");
            display_chain_choice()
        }
    }
}

pub fn display_action_choice() -> Result<Action> {
    let actions = vec![Action::Transfert, Action::DeFi, Action::NFT];
    let action_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an action")
        .items(&actions)
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap();

    match action_selection {
        Some(index) => return Ok(actions[index].clone()),
        None => println!("You have not selected anything"),
    }

    Ok(Action::default())
}

pub fn display_dex_choice(chain: Chain) -> Result<()> {
    let dexes = get_all_dexes(chain.as_ref()).unwrap();
    let dex_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a dex")
        .items(&dexes)
        .default(0)
        .interact_on_opt(&Term::stderr()).unwrap();
    
    match dex_selection {
        Some(index) => println!("User selected dex : {}", dexes[index]),
        None => println!("User did not select anything")
    }
    Ok(())
}

pub fn run() -> Result<()> {
    let chain = display_chain_choice()?;
    
    match display_action_choice()? {
        Action::Transfert => println!("Transfert"),
        Action::DeFi => display_dex_choice(chain)?,
        Action::NFT => println!("NFT"),
    }

    Ok(())
}