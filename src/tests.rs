#[cfg(test)]
use crate::calls;
#[cfg(test)]
use ethers::types::U256;
#[cfg(test)]
use eyre::Result;

#[cfg(test)]
const PRIVATE_KEY: &str = "2a671944041962a4817a31e4019d0cd2ba1ebf05161145ed54b617f9cb661ef5";
#[cfg(test)]
const CONTRACT_ADDRESS: &str = "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b";
#[cfg(test)]
const ACCOUNT_ADDRESS: &str = "0xbE6157bC090536ee15763356Ac11be00b15951E3";

#[tokio::test]
async fn test_total_supply() -> Result<()> {
    let _result = calls::total_supply(PRIVATE_KEY, CONTRACT_ADDRESS).await?;
    Ok(())
}

#[tokio::test]
async fn test_contract_type() -> Result<()> {
    let _result = calls::contract_type(PRIVATE_KEY, CONTRACT_ADDRESS).await?;
    Ok(())
}

#[tokio::test]
async fn test_allowance() -> Result<()> {
    let _result = calls::allowance(
        PRIVATE_KEY,
        CONTRACT_ADDRESS,
        ACCOUNT_ADDRESS,
        ACCOUNT_ADDRESS,
    )
    .await?;
    Ok(())
}

#[tokio::test]
async fn test_token_name() -> Result<()> {
    let _result = calls::name(PRIVATE_KEY, CONTRACT_ADDRESS).await?;
    Ok(())
}

#[tokio::test]
async fn test_token_symbol() -> Result<()> {
    let _result = calls::symbol(PRIVATE_KEY, CONTRACT_ADDRESS).await?;
    Ok(())
}

#[tokio::test]
async fn test_mint_tokens() -> Result<()> {
    let _result = calls::mint_to(
        PRIVATE_KEY,
        CONTRACT_ADDRESS,
        ACCOUNT_ADDRESS,
        U256::from(10000000000000000000000 as u128),
    )
    .await?;
    Ok(())
}

#[tokio::test]
async fn test_increase_allowance() -> Result<()> {
    let _result = calls::increase_allowance(
        PRIVATE_KEY,
        CONTRACT_ADDRESS,
        ACCOUNT_ADDRESS,
        U256::from(10000000000000000000000 as u128),
    )
    .await?;
    Ok(())
}
#[tokio::test]
async fn test_decrease_allowance() -> Result<()> {
    let _result = calls::decrease_allowance(
        PRIVATE_KEY,
        CONTRACT_ADDRESS,
        ACCOUNT_ADDRESS,
        U256::from(10000000000000000000000 as i128),
    )
    .await?;
    Ok(())
}
#[tokio::test]
async fn test_burn_tokens() -> Result<()> {
    let _result = calls::burn_from(
        PRIVATE_KEY,
        CONTRACT_ADDRESS,
        ACCOUNT_ADDRESS,
        U256::from(2757000000000110000000 as u128),
    )
    .await?;
    Ok(())
}
#[tokio::test]
async fn test_transfer() -> Result<()> {
    let _result = calls::transfer(
        PRIVATE_KEY,
        CONTRACT_ADDRESS,
        ACCOUNT_ADDRESS,
        U256::from(5000000000000000000000 as u128),
    )
    .await?;
    Ok(())
}
