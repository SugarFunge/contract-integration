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

    let wallet: LocalWallet = env.private_key.as_str().parse()?;
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

    let wallet: LocalWallet = env.private_key.as_str().parse()?;
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

pub async fn allowance(owner_address: &str, spender_address: &str) -> Result<AllowanceOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.as_str().parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");
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
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(AllowanceOutput { allowance: value })
}

pub async fn name() -> Result<NameOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.as_str().parse()?;
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

    let wallet: LocalWallet = env.private_key.as_str().parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");
    let value: String = contract.method::<_, _>("symbol", ())?.call().await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(SymbolOutput { symbol: value })
}

pub async fn mint_to(account_address: &str, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();

    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.as_str().parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");

    let mut tx = contract
        .method::<_, (Address, U256)>("mintTo", (account_address.parse::<Address>()?, amount))?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);
    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn increase_allowance(spender_address: &str, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.as_str().parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");

    let mut tx = contract.method::<_, (Address, U256)>(
        "increaseAllowance",
        (spender_address.parse::<Address>()?, amount),
    )?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);
    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn decrease_allowance(spender_address: &str, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.as_str().parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");

    let mut tx = contract.method::<_, (Address, U256)>(
        "decreaseAllowance",
        (spender_address.parse::<Address>()?, amount),
    )?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);
    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn burn_from(account_address: &str, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.as_str().parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");

    let mut tx = contract
        .method::<_, (Address, U256)>("burnFrom", (account_address.parse::<Address>()?, amount))?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);
    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}

pub async fn transfer(account_address: &str, amount: U256) -> Result<ReceiptOutput> {
    dotenv().ok();
    let env = config::init();
    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    let provider = GOERLI.provider();

    let wallet: LocalWallet = env.private_key.as_str().parse()?;
    let wallet = wallet.with_chain_id(CHAIN_ID);

    let client = SignerMiddleware::new(provider, wallet);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");

    let mut tx = contract
        .method::<_, (Address, U256)>("transfer", (account_address.parse::<Address>()?, amount))?;
    tx.tx.set_chain_id(CHAIN_ID);

    let pending_tx = tx.send().await?;
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);
    Ok(ReceiptOutput {
        receipt: receipt.unwrap(),
    })
}
