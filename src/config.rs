use std::env;
use ethers_core::{
    abi::{Abi, Uint},
    k256::{
        ecdsa::SigningKey,
        pkcs8::{AlgorithmIdentifier, ObjectIdentifier, PrivateKeyInfo},
    },
    types::{Address, H256, U256},
};

#[derive(Clone, Debug)]
pub struct Config {
    pub contract_address: Address,  
    pub account_address: Address,
    pub private_key: String,
}

pub fn init() -> Config { 
    let panic_message: String = "enviroment variable is not set".to_string();

    Config {
        contract_address: match env::var("CONTRACT_ADDRESS") {
            Ok(var) => var.parse::<Address>().unwrap(),
            Err(_) => panic!("CONTRACT_ADDRESS {}", panic_message)
        },
        account_address: match env::var("ACCOUNT_ADDRESS") {
            Ok(var) => var.parse::<Address>().unwrap(),
            Err(_) => panic!("ACCOUNT_ADDRESS {}", panic_message)
        },
        private_key: match env::var("PRIVATE_KEY") {
            Ok(var) => var,
            Err(_) => panic!("PRIVATE_KEY {}", panic_message)
        },
    }
}
