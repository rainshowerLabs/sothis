use serde::{Deserialize, Serialize};
use ethers::core::utils::rlp::*;

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code, non_snake_case)]
pub struct BlockResult {
    difficulty: String,
    extraData: String,
    gasLimit: String,
    gasUsed: String,
    hash: String,
    logsBloom: String,
    miner: String,
    mixHash: String,
    nonce: String,
    number: String,
    parentHash: String,
    receiptsRoot: String,
    sha3Uncles: String,
    size: String,
    stateRoot: String,
    pub timestamp: String,
    totalDifficulty: String,
    pub transactions: Vec<Transaction>,
    transactionsRoot: String,
    uncles: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code, non_snake_case)]
pub struct Transaction {
    pub blockHash: String,
    pub blockNumber: String,
    pub from: String,
    pub gas: String,
    pub gasPrice: String,
    pub hash: String,
    pub input: String,
    pub nonce: String,
    pub r: String,
    pub s: String,
    pub to: Option<String>,
    pub transactionIndex: String,
    #[serde(rename = "type")]
    pub txType: String,
    pub v: String,
    pub value: String,
}

impl rlp::Encodable for Transaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        // 0) `nonce`
        // 1) `gas_price`
        // 2) `gas_limit`
        // 3) `to`
        // 4) `value`
        // 5) `data`
        // 6) `v`
        // 7) `r`
        // 8) `s`

        s.begin_list(9)
            .append(&self.nonce)
            .append(&self.gasPrice)
            .append(&self.gas)
            .append(&self.to)
            .append(&self.value)
            .append(&self.input)
            .append(&self.v)
            .append(&self.r)
            .append(&self.s);
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TransactionParams {
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub gas: String,
    pub gasPrice: String,
    pub data: String,
    pub nonce: String,
    pub chainId: Option<String>,
}
