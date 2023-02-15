use dotenv::dotenv;
use ethers_core::types::{Address, U256};
use eyre::Result;

mod abi_file;
mod calls;
mod config;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let env = config::init();

    //-------------------------------------------CONSTANTS---------------------------------------------------------------

    let transfer_address = "0xbE6157bC090536ee15763356Ac11be00b15951E3".parse::<Address>()?;

    //-------------------------------------------SUCCESSFULL TESTS-------------------------------------------------------

    let result = calls::total_supply(env.contract_address).await?;
    println!("TOTAL SUPPLY: {}", result.total_supply);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::contract_type(env.contract_address).await?;
    println!("CONTRACT TYPE: {}", result.contract_type);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::allowance(
        env.contract_address,
        env.account_address,
        env.account_address,
    )
    .await?;
    println!("ALLOWANCE: {}", result.allowance);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::name(env.contract_address).await?;
    println!("TOKEN CONTRACT NAME: {}", result.name);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::symbol(env.contract_address).await?;
    println!("TOKEN CONTRACT SYMBOL: {}", result.symbol);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::mint_to(
        "a0554bccb5a4cbd6e2f74754f66deee5410b67806361e629d1a71299abc8f6db",
        env.contract_address,
        env.account_address,
        U256::from(10000000000000000000000 as i128),
    )
    .await?;
    println!("MINT RECEIPT: {:#?}", result.receipt);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::increase_allowance(
        "a0554bccb5a4cbd6e2f74754f66deee5410b67806361e629d1a71299abc8f6db",
        env.contract_address,
        env.account_address,
        U256::from(10000000000000000000000 as i128),
    )
    .await?;
    println!("INCREASE ALLOWANCE RECEIPT {:#?}", result.receipt);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::decrease_allowance(
        "a0554bccb5a4cbd6e2f74754f66deee5410b67806361e629d1a71299abc8f6db",
        env.contract_address,
        env.account_address,
        U256::from(6000000000000000000000 as i128),
    )
    .await?;
    println!("DECREASE ALLOWANCE RECEIPT {:#?}", result.receipt);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::burn_from(
        "a0554bccb5a4cbd6e2f74754f66deee5410b67806361e629d1a71299abc8f6db",
        env.contract_address,
        env.account_address,
        U256::from(3000000000000000000000 as i128),
    )
    .await?;
    println!("BURN RECEIPT: {:#?}", result.receipt);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::transfer(
        "a0554bccb5a4cbd6e2f74754f66deee5410b67806361e629d1a71299abc8f6db",
        env.contract_address,
        transfer_address,
        U256::from(2500000000000000000000 as i128),
    )
    .await?;
    println!("TRANSFER RECEIPT: {:#?}", result.receipt);

    Ok(())
}
