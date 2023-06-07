use crate::abi_file;
use crate::config;
use crate::main_calls::*;
use crate::types::*;
use dotenv::dotenv;
use ethers::core::types::U256;
use eyre::Result;

pub async fn goerli_total_supply() -> Result<TotalSupplyOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_goerli().abi;

    total_supply(
        &env.private_key,
        &env.goerli_contract_address,
        &env.goerli_url_id,
        5,
        abi,
    )
    .await
}

pub async fn goerli_contract_type() -> Result<ContractTypeOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_goerli().abi;

    contract_type(
        &env.private_key,
        &env.goerli_contract_address,
        &env.goerli_url_id,
        5,
        abi,
    )
    .await
}

pub async fn goerli_allowance(
    owner_address: &str,
    spender_address: &str,
) -> Result<AllowanceOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_goerli().abi;

    allowance(
        &env.private_key,
        &env.goerli_contract_address,
        owner_address,
        spender_address,
        &env.goerli_url_id,
        5,
        abi,
    )
    .await
}

pub async fn goerli_name() -> Result<NameOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_goerli().abi;

    name(
        &env.private_key,
        &env.goerli_contract_address,
        &env.goerli_url_id,
        5,
        abi,
    )
    .await
}

pub async fn goerli_symbol() -> Result<SymbolOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_goerli().abi;

    symbol(
        &env.private_key,
        &env.goerli_contract_address,
        &env.goerli_url_id,
        5,
        abi,
    )
    .await
}

pub async fn goerli_mint_to(account_address: &str, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_goerli().abi;

    mint_to(
        &env.private_key,
        &env.goerli_contract_address,
        account_address,
        amount,
        &env.goerli_url_id,
        5,
        abi,
        env.number_confirmations,
    )
    .await
}

pub async fn goerli_increase_allowance(
    spender_address: &str,
    amount: U256,
) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_goerli().abi;

    increase_allowance(
        &env.private_key,
        &env.goerli_contract_address,
        spender_address,
        amount,
        &env.goerli_url_id,
        5,
        abi,
        env.number_confirmations,
    )
    .await
}

pub async fn goerli_decrease_allowance(
    spender_address: &str,
    amount: U256,
) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_goerli().abi;

    decrease_allowance(
        &env.private_key,
        &env.goerli_contract_address,
        spender_address,
        amount,
        &env.goerli_url_id,
        5,
        abi,
        env.number_confirmations,
    )
    .await
}

pub async fn goerli_burn_from(account_address: &str, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_goerli().abi;

    burn_from(
        &env.private_key,
        &env.goerli_contract_address,
        account_address,
        amount,
        &env.goerli_url_id,
        5,
        abi,
        env.number_confirmations,
    )
    .await
}

pub async fn goerli_transfer(account_address: &str, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_goerli().abi;

    transfer(
        &env.private_key,
        &env.goerli_contract_address,
        account_address,
        amount,
        &env.goerli_url_id,
        5,
        abi,
        env.number_confirmations,
    )
    .await
}

pub async fn mumbai_total_supply() -> Result<TotalSupplyOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_mumbai().abi;

    total_supply(
        &env.private_key,
        &env.mumbai_contract_address,
        &env.mumbai_url_id,
        80001,
        abi,
    )
    .await
}

pub async fn mumbai_contract_type() -> Result<ContractTypeOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_mumbai().abi;

    contract_type(
        &env.private_key,
        &env.mumbai_contract_address,
        &env.mumbai_url_id,
        80001,
        abi,
    )
    .await
}

pub async fn mumbai_allowance(
    owner_address: &str,
    spender_address: &str,
) -> Result<AllowanceOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_mumbai().abi;

    allowance(
        &env.private_key,
        &env.mumbai_contract_address,
        owner_address,
        spender_address,
        &env.mumbai_url_id,
        80001,
        abi,
    )
    .await
}

pub async fn mumbai_name() -> Result<NameOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_mumbai().abi;

    name(
        &env.private_key,
        &env.mumbai_contract_address,
        &env.mumbai_url_id,
        80001,
        abi,
    )
    .await
}

pub async fn mumbai_symbol() -> Result<SymbolOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_mumbai().abi;

    symbol(
        &env.private_key,
        &env.mumbai_contract_address,
        &env.mumbai_url_id,
        80001,
        abi,
    )
    .await
}

pub async fn mumbai_mint_to(account_address: &str, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_mumbai().abi;

    mint_to(
        &env.private_key,
        &env.mumbai_contract_address,
        account_address,
        amount,
        &env.mumbai_url_id,
        80001,
        abi,
        env.number_confirmations,
    )
    .await
}

pub async fn mumbai_increase_allowance(
    spender_address: &str,
    amount: U256,
) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_mumbai().abi;

    increase_allowance(
        &env.private_key,
        &env.mumbai_contract_address,
        spender_address,
        amount,
        &env.mumbai_url_id,
        80001,
        abi,
        env.number_confirmations,
    )
    .await
}

pub async fn mumbai_decrease_allowance(
    spender_address: &str,
    amount: U256,
) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_mumbai().abi;

    decrease_allowance(
        &env.private_key,
        &env.mumbai_contract_address,
        spender_address,
        amount,
        &env.mumbai_url_id,
        80001,
        abi,
        env.number_confirmations,
    )
    .await
}

pub async fn mumbai_burn_from(account_address: &str, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_mumbai().abi;

    burn_from(
        &env.private_key,
        &env.mumbai_contract_address,
        account_address,
        amount,
        &env.mumbai_url_id,
        80001,
        abi,
        env.number_confirmations,
    )
    .await
}

pub async fn mumbai_transfer(account_address: &str, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init_mumbai().abi;

    transfer(
        &env.private_key,
        &env.mumbai_contract_address,
        account_address,
        amount,
        &env.mumbai_url_id,
        80001,
        abi,
        env.number_confirmations,
    )
    .await
}
