use ethers::core::types::Address;
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub contract_address: Address,
    pub account_address: Address,
    pub private_key: String,
    pub chain_id: u64,
    pub url_id: String,
    pub number_confirmations: usize,
}

pub fn init() -> Config {
    let panic_message: String = "enviroment variable is not set".to_string();

    Config {
        contract_address: match env::var("CONTRACT_ADDRESS") {
            Ok(var) => var.parse::<Address>().unwrap(),
            Err(_) => panic!("CONTRACT_ADDRESS {}", panic_message),
        },
        account_address: match env::var("ACCOUNT_ADDRESS") {
            Ok(var) => var.parse::<Address>().unwrap(),
            Err(_) => panic!("ACCOUNT_ADDRESS {}", panic_message),
        },
        private_key: match env::var("PRIVATE_KEY") {
            Ok(var) => var,
            Err(_) => panic!("PRIVATE_KEY {}", panic_message),
        },
        chain_id: match env::var("CHAIN_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("CHAIN_ID {}", panic_message),
        },
        url_id: match env::var("URL_ID") {
            Ok(var) => var,
            Err(_) => panic!("URL_ID {}", panic_message),
        },
        number_confirmations: match env::var("NUMBER_CONFIRMATIONS") {
            Ok(var) => var.parse::<usize>().unwrap(),
            Err(_) => panic!("NUMBER_CONFIRMATIONS {}", panic_message),
        },
    }
}
