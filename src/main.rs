use ethers_contract::Contract;
use ethers_core::{
    abi::{Abi, Uint},
    k256::{
        ecdsa::SigningKey,
        pkcs8::{AlgorithmIdentifier, ObjectIdentifier, PrivateKeyInfo},
    },
    types::{Address, H256, U256},
};
use ethers_middleware::signer::SignerMiddleware;
use ethers_providers::{Http, Middleware, Provider, GOERLI};
use ethers_signers::{coins_bip39::English, LocalWallet, MnemonicBuilder, Signer, Wallet};
use eyre::Result;
use std::env;
use dotenv::dotenv;

mod abi_file;
mod calls;
mod config;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    // -----------------------------------------------CONSTANTS ----------------------------------------------------------
    dotenv().ok();
    let env = config::init();

    // let contract_address = match env::var("CONTRACT_ADDRESS") {
    //     Ok(var) => var.parse::<Address>()?,
    //     Err(_) => panic!("CONTRACT_ADDRESS {}", panic_message)
    // };
    // let account_address = match env::var("ACCOUNT_ADDRESS") {
    //     Ok(var) => var.parse::<Address>()?,
    //     Err(_) => panic!("ACCOUNT_ADDRESS {}", panic_message)
    // };
    // let private_key = match env::var("PRIVATE_KEY") {
    //     Ok(var) => var,
    //     Err(_) => panic!("PRIVATE_KEY {}", panic_message)
    // };
    
    // let contract_address = "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b".parse::<Address>()?;

    // let account_address = "0xbE6157bC090536ee15763356Ac11be00b15951E3".parse::<Address>()?;

    //-------------------------------------------SUCCESSFULL TESTS-------------------------------------------------------

    // let result = calls::total_supply(contract_address).await?;
    // println!("TOTAL SUPPLY: {}", result.total_supply);

    // let result = calls::contract_type(contract_address).await?;
    // println!("CONTRACT TYPE: {}", result.contract_type);

    // let result = calls::allowance(contract_address, account_address, account_address).await?;
    // println!("ALLOWANCE: {}", result.allowance);

    // let result = calls::name(contract_address).await?;
    // println!("TOKEN CONTRACT NAME: {}", result.name);

    // let result = calls::symbol(contract_address).await?;
    // println!("TOKEN CONTRACT SYMBOL: {}", result.symbol);

    //--------------------------------------------TESTING HOW CAN WE MADE SIGN TRANSACTIONS WITH METAMASK ACCOUNT---------------------

    let abi = abi_file::init().abi;
    println!("1. OBTENIDO EL ABI");

    println!("{:#?}", env);

    let provider = GOERLI.provider();
    let wallet: LocalWallet = env.private_key.parse()?;

    let wallet = wallet.with_chain_id(5 as u64);

    let client = SignerMiddleware::new(provider, wallet);

    let ba = client.get_chainid().await.unwrap();
    println!("EL CHAIN ID DEL CLIENT ES: {:#?}", ba);

    let contract = Contract::new(env.contract_address, abi, client);
    println!("2. CREADO EL CONTRATO");

    // Non-constant methods are executed via the `send()` call on the method builder.
    let mut tx =
        contract.method::<_, (Address, U256)>("mintTo", (env.account_address, U256::from(10000000)))?;

    tx.tx.set_chain_id(5);

    let pending_tx = tx.send().await?;

    // You can `await` on the pending transaction to get the receipt with a pre-specified
    // number of confirmations
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);

    //----------------------------------------------------------------------------------------------------------------------------------

    Ok(())
}
