use crate::abi_file;
use crate::types::*;
use ethers::contract::Contract;
use ethers::core::types::U256;
use ethers::core::{types::Address, utils::parse_bytes32_string};
use ethers::middleware::signer::SignerMiddleware;
use ethers::providers::Provider;
use ethers::signers::LocalWallet;
use ethers::signers::Signer;
use eyre::Result;

// This represents the provider chain in this case Goerli
const CHAIN_ID: u64 = 5;
// This representen the URL of the Ethereum Node
const URL_ID: &str = "https://nd-776-423-051.p2pify.com/e34b9f9ebf64e6dadaa4a2774a1e5d74";

pub async fn total_supply(private_key: &str, contract_address: &str) -> Result<TotalSupplyOutput> {
    let abi = abi_file::init().abi;

    let provider = Provider::try_from(URL_ID)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

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
) -> Result<ContractTypeOutput> {
    let abi = abi_file::init().abi;

    let provider = Provider::try_from(URL_ID)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

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
) -> Result<AllowanceOutput> {
    let abi = abi_file::init().abi;

    let provider = Provider::try_from(URL_ID)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

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

pub async fn name(private_key: &str, contract_address: &str) -> Result<NameOutput> {
    let abi = abi_file::init().abi;

    let provider = Provider::try_from(URL_ID)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let value: String = contract.method::<_, _>("name", ())?.call().await?;

    Ok(NameOutput { name: value })
}

pub async fn symbol(private_key: &str, contract_address: &str) -> Result<SymbolOutput> {
    let abi = abi_file::init().abi;

    let provider = Provider::try_from(URL_ID)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

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
) -> Result<ReceiptOutput> {
    let abi = abi_file::init().abi;

    let provider = Provider::try_from(URL_ID)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let mut tx = contract
        .method::<_, (Address, U256)>("mintTo", (account_address.parse::<Address>()?, amount))?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn increase_allowance(
    private_key: &str,
    contract_address: &str,
    spender_address: &str,
    amount: U256,
) -> Result<ReceiptOutput> {
    let abi = abi_file::init().abi;

    let provider = Provider::try_from(URL_ID)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let mut tx = contract.method::<_, (Address, U256)>(
        "increaseAllowance",
        (spender_address.parse::<Address>()?, amount),
    )?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn decrease_allowance(
    private_key: &str,
    contract_address: &str,
    spender_address: &str,
    amount: U256,
) -> Result<ReceiptOutput> {
    let abi = abi_file::init().abi;

    let provider = Provider::try_from(URL_ID)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let mut tx = contract.method::<_, (Address, U256)>(
        "decreaseAllowance",
        (spender_address.parse::<Address>()?, amount),
    )?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn burn_from(
    private_key: &str,
    contract_address: &str,
    account_address: &str,
    amount: U256,
) -> Result<ReceiptOutput> {
    let abi = abi_file::init().abi;

    let provider = Provider::try_from(URL_ID)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let mut tx = contract
        .method::<_, (Address, U256)>("burnFrom", (account_address.parse::<Address>()?, amount))?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn transfer(
    private_key: &str,
    contract_address: &str,
    account_address: &str,
    amount: U256,
) -> Result<ReceiptOutput> {
    let abi = abi_file::init().abi;

    let provider = Provider::try_from(URL_ID)?;

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address.parse::<Address>()?, abi, client);

    let mut tx = contract
        .method::<_, (Address, U256)>("transfer", (account_address.parse::<Address>()?, amount))?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}
