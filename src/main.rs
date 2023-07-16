use airdrop_farmoooooor::{runner::*, utils::constants::get_banner};
use eyre::Result;

fn main() -> Result<()> {
    println!("{}", get_banner());
    run()?;
    Ok(())
}