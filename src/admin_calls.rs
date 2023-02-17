use crate::abi_file;
use crate::config;
use crate::types::*;
use dotenv::dotenv;
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

pub async fn total_supply() -> Result<TotalSupplyOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");
    let value = contract.method::<_, _>("totalSupply", ())?.call().await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(TotalSupplyOutput {
        total_supply: value,
    })
}

pub async fn contract_type() -> Result<ContractTypeOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
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
    owner_address: Address,
    spender_address: Address,
) -> Result<AllowanceOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");
    let value = contract
        .method::<_, _>("allowance", (owner_address, spender_address))?
        .call()
        .await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(AllowanceOutput { allowance: value })
}

pub async fn name() -> Result<NameOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");
    let value: String = contract.method::<_, _>("name", ())?.call().await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(NameOutput { name: value })
}

pub async fn symbol() -> Result<SymbolOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");
    let value: String = contract.method::<_, _>("symbol", ())?.call().await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(SymbolOutput { symbol: value })
}

pub async fn mint_to(account_address: Address, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();

    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
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

pub async fn increase_allowance(spender_address: Address, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
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

pub async fn decrease_allowance(spender_address: Address, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
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

pub async fn burn_from(account_address: Address, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
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

pub async fn transfer(account_address: Address, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
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
