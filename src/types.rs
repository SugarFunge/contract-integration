use ethers::core::{abi::Uint, types::TransactionReceipt};

pub struct TotalSupplyOutput {
    pub total_supply: Uint,
}

pub struct ContractTypeOutput {
    pub contract_type: String,
}

pub struct AllowanceOutput {
    pub allowance: Uint,
}

pub struct NameOutput {
    pub name: String,
}

pub struct SymbolOutput {
    pub symbol: String,
}

pub struct ReceiptOutput {
    pub receipt: TransactionReceipt,
}
