use ethers::core::{abi::Uint, types::TransactionReceipt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TotalSupplyOutput {
    pub total_supply: Uint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractTypeOutput {
    pub contract_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllowanceOutput {
    pub allowance: Uint,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NameOutput {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolOutput {
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiptOutput {
    pub receipt: TransactionReceipt,
}
