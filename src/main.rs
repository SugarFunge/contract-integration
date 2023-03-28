use dotenv::dotenv;
use ethers::core::types::U256;
use eyre::Result;

mod abi_file;
mod admin_calls;
mod calls;
mod config;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let env = config::init();

    // //-------------------------------------------SUCCESSFULL TESTS-------------------------------------------------------

    let result = calls::total_supply(
        "2a671944041962a4817a31e4019d0cd2ba1ebf05161145ed54b617f9cb661ef5",
        "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b",
    )
    .await?;
    println!("TOTAL SUPPLY: {}", result.total_supply);

    let result = admin_calls::total_supply().await?;
    println!("TOTAL SUPPLY: {}", result.total_supply);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::contract_type(
        "2a671944041962a4817a31e4019d0cd2ba1ebf05161145ed54b617f9cb661ef5",
        "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b",
    )
    .await?;
    println!("CONTRACT TYPE: {}", result.contract_type);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::allowance(
        "2a671944041962a4817a31e4019d0cd2ba1ebf05161145ed54b617f9cb661ef5",
        "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b",
        "0xbE6157bC090536ee15763356Ac11be00b15951E3",
        "0xbE6157bC090536ee15763356Ac11be00b15951E3",
    )
    .await?;
    println!("ALLOWANCE: {}", result.allowance);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::name(
        "2a671944041962a4817a31e4019d0cd2ba1ebf05161145ed54b617f9cb661ef5",
        "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b",
    )
    .await?;
    println!("TOKEN CONTRACT NAME: {}", result.name);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::symbol(
        "2a671944041962a4817a31e4019d0cd2ba1ebf05161145ed54b617f9cb661ef5",
        "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b",
    )
    .await?;
    println!("TOKEN CONTRACT SYMBOL: {}", result.symbol);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::mint_to(
        "2a671944041962a4817a31e4019d0cd2ba1ebf05161145ed54b617f9cb661ef5",
        "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b",
        "0xbE6157bC090536ee15763356Ac11be00b15951E3",
        U256::from(10000000000000000000000 as u128),
    )
    .await?;
    println!("MINT RECEIPT: {:#?}", result.receipt);

    let result = admin_calls::mint_to(
        "0xbE6157bC090536ee15763356Ac11be00b15951E3",
        U256::from(10000000000000000000000 as u128),
    )
    .await?;
    println!("MINT RECEIPT: {:#?}", result.receipt);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::increase_allowance(
        "2a671944041962a4817a31e4019d0cd2ba1ebf05161145ed54b617f9cb661ef5",
        "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b",
        "0xbE6157bC090536ee15763356Ac11be00b15951E3",
        U256::from(10000000000000000000000 as u128),
    )
    .await?;
    println!("INCREASE ALLOWANCE RECEIPT {:#?}", result.receipt);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::decrease_allowance(
        "2a671944041962a4817a31e4019d0cd2ba1ebf05161145ed54b617f9cb661ef5",
        "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b",
        "0xbE6157bC090536ee15763356Ac11be00b15951E3",
        U256::from(10000000000000000000000 as i128),
    )
    .await?;
    println!("DECREASE ALLOWANCE RECEIPT {:#?}", result.receipt);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::burn_from(
        "2a671944041962a4817a31e4019d0cd2ba1ebf05161145ed54b617f9cb661ef5",
        "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b",
        "0xbE6157bC090536ee15763356Ac11be00b15951E3",
        U256::from(2757000000000110000000 as u128),
    )
    .await?;
    println!("BURN RECEIPT: {:#?}", result.receipt);

    //--------------------------------------------------------------------------------------------------------------------

    let result = calls::transfer(
        "2a671944041962a4817a31e4019d0cd2ba1ebf05161145ed54b617f9cb661ef5",
        "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b",
        "0x768e02d0b50fcBc97163CBe70285236e97Ff3001",
        U256::from(5000000000000000000000 as u128),
    )
    .await?;
    println!("TRANSFER RECEIPT: {:#?}", result.receipt);

    Ok(())
}
