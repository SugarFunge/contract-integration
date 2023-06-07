use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub account_address: String,
    pub private_key: String,
    pub goerli_contract_address: String,
    pub goerli_url_id: String,
    pub mumbai_contract_address: String,
    pub mumbai_url_id: String,
    pub number_confirmations: usize,
}

pub fn init() -> Config {
    let panic_message: String = "enviroment variable is not set".to_string();

    Config {
        account_address: match env::var("ACCOUNT_ADDRESS") {
            Ok(var) => var,
            Err(_) => panic!("ACCOUNT_ADDRESS {}", panic_message),
        },
        private_key: match env::var("ACCOUNT_PRIVATE_KEY") {
            Ok(var) => var,
            Err(_) => panic!("ACCOUNT_PRIVATE_KEY {}", panic_message),
        },
        goerli_contract_address: match env::var("GOERLI_CONTRACT_ADDRESS") {
            Ok(var) => var,
            Err(_) => panic!("GOERLI_CONTRACT_ADDRESS {}", panic_message),
        },
        goerli_url_id: match env::var("GOERLI_URL_ID") {
            Ok(var) => var,
            Err(_) => panic!("GOERLI_URL_ID {}", panic_message),
        },
        mumbai_contract_address: match env::var("MUMBAI_CONTRACT_ADDRESS") {
            Ok(var) => var,
            Err(_) => panic!("MUMBAI_CONTRACT_ADDRESS {}", panic_message),
        },
        mumbai_url_id: match env::var("MUMBAI_URL_ID") {
            Ok(var) => var,
            Err(_) => panic!("MUMBAI_URL_ID {}", panic_message),
        },
        number_confirmations: match env::var("NUMBER_CONFIRMATIONS") {
            Ok(var) => var.parse::<usize>().unwrap(),
            Err(_) => panic!("NUMBER_CONFIRMATIONS {}", panic_message),
        },
    }
}
