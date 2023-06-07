use crate::types::*;
use ethers::contract::Contract;
use ethers::core::abi::Abi;
use ethers::core::types::U256;
use ethers::core::{types::Address, utils::parse_bytes32_string};
use ethers::middleware::signer::SignerMiddleware;
use ethers::providers::Provider;
use ethers::signers::LocalWallet;
use ethers::signers::Signer;
use eyre::Result;

pub async fn total_supply(
    private_key: &str,
    contract_address: &str,
    url_id: &str,
    chain_id: u64,
    abi: Abi,
) -> Result<TotalSupplyOutput> {
    let provider = Provider::try_from(url_id)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let value = contract.method::<_, _>("totalSupply", ())?.call().await?;

    Ok(TotalSupplyOutput {
        total_supply: value,
    })
}

pub async fn contract_type(
    private_key: &str,
    contract_address: &str,
    url_id: &str,
    chain_id: u64,
    abi: Abi,
) -> Result<ContractTypeOutput> {
    let provider = Provider::try_from(url_id)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let value: [u8; 32] = contract.method::<_, _>("contractType", ())?.call().await?;

    Ok(ContractTypeOutput {
        contract_type: String::from(parse_bytes32_string(&value).unwrap()),
    })
}

pub async fn allowance(
    private_key: &str,
    contract_address: &str,
    owner_address: &str,
    spender_address: &str,
    url_id: &str,
    chain_id: u64,
    abi: Abi,
) -> Result<AllowanceOutput> {
    let provider = Provider::try_from(url_id)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let value = contract
        .method::<_, _>(
            "allowance",
            (
                owner_address.parse::<Address>()?,
                spender_address.parse::<Address>()?,
            ),
        )?
        .call()
        .await?;

    Ok(AllowanceOutput { allowance: value })
}

pub async fn name(
    private_key: &str,
    contract_address: &str,
    url_id: &str,
    chain_id: u64,
    abi: Abi,
) -> Result<NameOutput> {
    let provider = Provider::try_from(url_id)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let value: String = contract.method::<_, _>("name", ())?.call().await?;

    Ok(NameOutput { name: value })
}

pub async fn symbol(
    private_key: &str,
    contract_address: &str,
    url_id: &str,
    chain_id: u64,
    abi: Abi,
) -> Result<SymbolOutput> {
    let provider = Provider::try_from(url_id)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let value: String = contract.method::<_, _>("symbol", ())?.call().await?;

    Ok(SymbolOutput { symbol: value })
}

pub async fn mint_to(
    private_key: &str,
    contract_address: &str,
    account_address: &str,
    amount: U256,
    url_id: &str,
    chain_id: u64,
    abi: Abi,
    number_confirmations: usize,
) -> Result<ReceiptOutput> {
    let provider = Provider::try_from(url_id)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let mut tx = contract
        .method::<_, (Address, U256)>("mintTo", (account_address.parse::<Address>()?, amount))?;
    tx.tx.set_chain_id(chain_id);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(number_confirmations).await?;

    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn increase_allowance(
    private_key: &str,
    contract_address: &str,
    spender_address: &str,
    amount: U256,
    url_id: &str,
    chain_id: u64,
    abi: Abi,
    number_confirmations: usize,
) -> Result<ReceiptOutput> {
    let provider = Provider::try_from(url_id)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let mut tx = contract.method::<_, (Address, U256)>(
        "increaseAllowance",
        (spender_address.parse::<Address>()?, amount),
    )?;
    tx.tx.set_chain_id(chain_id);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(number_confirmations).await?;

    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn decrease_allowance(
    private_key: &str,
    contract_address: &str,
    spender_address: &str,
    amount: U256,
    url_id: &str,
    chain_id: u64,
    abi: Abi,
    number_confirmations: usize,
) -> Result<ReceiptOutput> {
    let provider = Provider::try_from(url_id)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let mut tx = contract.method::<_, (Address, U256)>(
        "decreaseAllowance",
        (spender_address.parse::<Address>()?, amount),
    )?;
    tx.tx.set_chain_id(chain_id);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(number_confirmations).await?;

    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn burn_from(
    private_key: &str,
    contract_address: &str,
    account_address: &str,
    amount: U256,
    url_id: &str,
    chain_id: u64,
    abi: Abi,
    number_confirmations: usize,
) -> Result<ReceiptOutput> {
    let provider = Provider::try_from(url_id)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let mut tx = contract
        .method::<_, (Address, U256)>("burnFrom", (account_address.parse::<Address>()?, amount))?;
    tx.tx.set_chain_id(chain_id);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(number_confirmations).await?;

    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn transfer(
    private_key: &str,
    contract_address: &str,
    account_address: &str,
    amount: U256,
    url_id: &str,
    chain_id: u64,
    abi: Abi,
    number_confirmations: usize,
) -> Result<ReceiptOutput> {
    let provider = Provider::try_from(url_id)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(chain_id);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let mut tx = contract
        .method::<_, (Address, U256)>("transfer", (account_address.parse::<Address>()?, amount))?;
    tx.tx.set_chain_id(chain_id);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(number_confirmations).await?;

    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}
