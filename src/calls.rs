use crate::abi_file;
use crate::types::*;
use ethers::contract::Contract;
use ethers::core::types::U256;
use ethers::core::{types::Address, utils::parse_bytes32_string};
use ethers::middleware::signer::SignerMiddleware;
use ethers::providers::GOERLI;
use ethers::signers::LocalWallet;
use ethers::signers::Signer;
use eyre::Result;

// This represents the provider chain in this case Goerli
const CHAIN_ID: u64 = 5;

pub async fn total_supply(contract_address: Address) -> Result<TotalSupplyOutput> {
    let abi = abi_file::init().abi;
    println!("\n1. OBTENIDO EL ABI");
    let contract = Contract::new(contract_address, abi, GOERLI.provider());
    println!("2. CREADO EL CONTRATO");
    let value = contract.method::<_, _>("totalSupply", ())?.call().await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(TotalSupplyOutput {
        total_supply: value,
    })
}

pub async fn contract_type(contract_address: Address) -> Result<ContractTypeOutput> {
    let abi = abi_file::init().abi;
    println!("\n1. OBTENIDO EL ABI");
    let contract = Contract::new(contract_address, abi, GOERLI.provider());
    println!("2. CREADO EL CONTRATO");
    let value: [u8; 32] = contract.method::<_, _>("contractType", ())?.call().await?;
    println!(
        "3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}",
        parse_bytes32_string(&value)
    );
    Ok(ContractTypeOutput {
        contract_type: String::from(parse_bytes32_string(&value).unwrap()),
    })
}

pub async fn allowance(
    contract_address: Address,
    owner_address: Address,
    spender_address: Address,
) -> Result<AllowanceOutput> {
    let abi = abi_file::init().abi;
    println!("\n1. OBTENIDO EL ABI");
    let contract = Contract::new(contract_address, abi, GOERLI.provider());
    println!("2. CREADO EL CONTRATO");
    let value = contract
        .method::<_, _>("allowance", (owner_address, spender_address))?
        .call()
        .await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(AllowanceOutput { allowance: value })
}

pub async fn name(contract_address: Address) -> Result<NameOutput> {
    let abi = abi_file::init().abi;
    println!("\n1. OBTENIDO EL ABI");
    let contract = Contract::new(contract_address, abi, GOERLI.provider());
    println!("2. CREADO EL CONTRATO");
    let value: String = contract.method::<_, _>("name", ())?.call().await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(NameOutput { name: value })
}

pub async fn symbol(contract_address: Address) -> Result<SymbolOutput> {
    let abi = abi_file::init().abi;
    println!("\n1. OBTENIDO EL ABI");
    let contract = Contract::new(contract_address, abi, GOERLI.provider());
    println!("2. CREADO EL CONTRATO");
    let value: String = contract.method::<_, _>("symbol", ())?.call().await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(SymbolOutput { symbol: value })
}

pub async fn mint_to(
    private_key: &str,
    contract_address: Address,
    account_address: Address,
    amount: U256,
) -> Result<ReceiptOutput> {
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");

    let mut tx = contract.method::<_, (Address, U256)>("mintTo", (account_address, amount))?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);
    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn increase_allowance(
    private_key: &str,
    contract_address: Address,
    spender_address: Address,
    amount: U256,
) -> Result<ReceiptOutput> {
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");

    let mut tx =
        contract.method::<_, (Address, U256)>("increaseAllowance", (spender_address, amount))?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);
    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn decrease_allowance(
    private_key: &str,
    contract_address: Address,
    spender_address: Address,
    amount: U256,
) -> Result<ReceiptOutput> {
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");

    let mut tx =
        contract.method::<_, (Address, U256)>("decreaseAllowance", (spender_address, amount))?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);
    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn burn_from(
    private_key: &str,
    contract_address: Address,
    account_address: Address,
    amount: U256,
) -> Result<ReceiptOutput> {
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");

    let mut tx = contract.method::<_, (Address, U256)>("burnFrom", (account_address, amount))?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);
    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn transfer(
    private_key: &str,
    contract_address: Address,
    account_address: Address,
    amount: U256,
) -> Result<ReceiptOutput> {
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");

    let mut tx = contract.method::<_, (Address, U256)>("transfer", (account_address, amount))?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);
    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}
