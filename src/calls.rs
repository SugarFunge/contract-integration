use crate::abi_file;
use crate::types::*;
use ethers_contract::Contract;
use ethers_core::{types::Address, utils::parse_bytes32_string};
use ethers_middleware::signer::SignerMiddleware;
use ethers_providers::GOERLI;
use ethers_signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Wallet};
use eyre::Result;

pub async fn total_supply(address: Address) -> Result<TotalSupplyOutput> {
    let abi = abi_file::init().abi;
    println!("\n1. OBTENIDO EL ABI");
    let contract = Contract::new(address, abi, GOERLI.provider());
    println!("2. CREADO EL CONTRATO");
    let value = contract.method::<_, _>("totalSupply", ())?.call().await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(TotalSupplyOutput {
        total_supply: value,
    })
}

pub async fn contract_type(address: Address) -> Result<ContractTypeOutput> {
    let abi = abi_file::init().abi;
    println!("\n1. OBTENIDO EL ABI");
    let contract = Contract::new(address, abi, GOERLI.provider());
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
    address: Address,
    owner: Address,
    spender: Address,
) -> Result<AllowanceOutput> {
    let abi = abi_file::init().abi;
    println!("\n1. OBTENIDO EL ABI");
    let contract = Contract::new(address, abi, GOERLI.provider());
    println!("2. CREADO EL CONTRATO");
    let value = contract
        .method::<_, _>("allowance", (owner, spender))?
        .call()
        .await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(AllowanceOutput { allowance: value })
}

pub async fn name(address: Address) -> Result<NameOutput> {
    let abi = abi_file::init().abi;
    println!("\n1. OBTENIDO EL ABI");
    let contract = Contract::new(address, abi, GOERLI.provider());
    println!("2. CREADO EL CONTRATO");
    let value: String = contract.method::<_, _>("name", ())?.call().await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(NameOutput { name: value })
}

pub async fn symbol(address: Address) -> Result<SymbolOutput> {
    let abi = abi_file::init().abi;
    println!("\n1. OBTENIDO EL ABI");
    let contract = Contract::new(address, abi, GOERLI.provider());
    println!("2. CREADO EL CONTRATO");
    let value: String = contract.method::<_, _>("symbol", ())?.call().await?;
    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {}", value);
    Ok(SymbolOutput { symbol: value })
}
