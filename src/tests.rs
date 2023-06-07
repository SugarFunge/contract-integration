#[cfg(test)]
use crate::calls;
#[cfg(test)]
use crate::config;
#[cfg(test)]
use dotenv::dotenv;
#[cfg(test)]
use ethers::types::U256;
#[cfg(test)]
use eyre::Result;

#[tokio::test]
async fn test_goerli_total_supply() -> Result<()> {
    let result = calls::goerli_total_supply().await?;
    println!("Total Supply Goerli: {}", result.total_supply);
    Ok(())
}

#[tokio::test]
async fn test_mumbai_total_supply() -> Result<()> {
    let result = calls::mumbai_total_supply().await?;
    println!("Total Supply Mumbai: {}", result.total_supply);
    Ok(())
}

#[tokio::test]
async fn test_goerli_contract_type() -> Result<()> {
    let _result = calls::goerli_contract_type().await?;
    Ok(())
}

#[tokio::test]
async fn test_goerli_allowance() -> Result<()> {
    dotenv().ok();
    let env = config::init();
    let _result = calls::goerli_allowance(&env.account_address, &env.account_address).await?;
    Ok(())
}

#[tokio::test]
async fn test_goerli_token_name() -> Result<()> {
    let _result = calls::goerli_name().await?;
    Ok(())
}

#[tokio::test]
async fn test_goerli_token_symbol() -> Result<()> {
    let _result = calls::goerli_symbol().await?;
    Ok(())
}

#[tokio::test]
async fn test_goerli_mint_tokens() -> Result<()> {
    dotenv().ok();
    let env = config::init();
    let _result = calls::goerli_mint_to(
        &env.account_address,
        U256::from(10000000000000000000000 as u128),
    )
    .await?;
    Ok(())
}

#[tokio::test]
async fn test_mumbai_mint_tokens() -> Result<()> {
    dotenv().ok();
    let env = config::init();
    let _result = calls::mumbai_mint_to(
        &env.account_address,
        U256::from(10000000000000000000000 as u128),
    )
    .await?;
    Ok(())
}

#[tokio::test]
async fn test_goerli_increase_allowance() -> Result<()> {
    dotenv().ok();
    let env = config::init();
    let _result = calls::goerli_increase_allowance(
        &env.account_address,
        U256::from(10000000000000000000000 as u128),
    )
    .await?;
    Ok(())
}
#[tokio::test]
async fn test_goerli_decrease_allowance() -> Result<()> {
    dotenv().ok();
    let env = config::init();
    let _result = calls::goerli_decrease_allowance(
        &env.account_address,
        U256::from(10000000000000000000000 as i128),
    )
    .await?;
    Ok(())
}
#[tokio::test]
async fn test_goerli_burn_tokens() -> Result<()> {
    dotenv().ok();
    let env = config::init();
    let _result = calls::goerli_burn_from(
        &env.account_address,
        U256::from(2757000000000110000000 as u128),
    )
    .await?;
    Ok(())
}
#[tokio::test]
async fn test_goerli_transfer() -> Result<()> {
    dotenv().ok();
    let env = config::init();
    let _result = calls::goerli_transfer(
        &env.account_address,
        U256::from(5000000000000000000000 as u128),
    )
    .await?;
    Ok(())
}
