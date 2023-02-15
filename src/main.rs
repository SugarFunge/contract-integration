use std::env;
use ethers_contract::Contract;
use ethers_core::{
    abi::{Abi, Uint},
    types::{Address, H256, U256},
    k256::{ecdsa::SigningKey, pkcs8::{PrivateKeyInfo, AlgorithmIdentifier, ObjectIdentifier}},
};
use ethers_providers::GOERLI;
use ethers_signers::{LocalWallet, Wallet, MnemonicBuilder, coins_bip39::English};
use eyre::Result;

mod abi;
mod calls;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    // -----------------------------------------------CONSTANTS ----------------------------------------------------------
    let contract_address = "0x5209A9A17e0A54615D3C24C92570fB5b9B14AB1b".parse::<Address>()?;

    let account_address = "0xbE6157bC090536ee15763356Ac11be00b15951E3".parse::<Address>()?;

    let abi = abi::init().abi;
    println!("1. OBTENIDO EL ABI");

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

    let contract = Contract::new(contract_address, abi, GOERLI.provider());
    println!("2. CREADO EL CONTRATO");

    // Non-constant methods are executed via the `send()` call on the method builder.
    let call = contract
        .method::<_, (Address, U256)>("mintTo", (account_address, U256::from(100000000)))?;

    let algorithm = AlgorithmIdentifier{ 
      oid: ObjectIdentifier::new_unwrap("0x0"),
      parameters: None
    };

    let private_key = env::var("PRIVATE_KEY")?;

    let private_key_info = PrivateKeyInfo::new(algorithm, private_key.as_bytes());
    let signature = SigningKey::try_from(private_key_info).unwrap();

    let wallet = Wallet::from(signature);

    // let wallet = MnemonicBuilder::<English>::default()
    // .phrase("aware butter obscure loop case private math wage buyer job enjoy winner")
    // .build()?;

    wallet.sign_transaction_sync(&call.tx);

    let pending_tx = call.send().await?;

    // `await`ing on the pending transaction resolves to a transaction receipt
    let receipt = pending_tx.confirmations(6).await?;

    println!("3. OBTENIDA LA RESPUESTA DEL ENDPOINT: {:#?}", receipt);

    //----------------------------------------------------------------------------------------------------------------------------------

    Ok(())
}
