use serde::{Deserialize, Serialize};
use ethers::utils::hex;

use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::Signature;
use ethers::types::U256;

use std::borrow::Cow;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code, non_snake_case)]
pub struct Transaction {
    pub blockHash: String,
    pub blockNumber: String,
    pub from: String,
    pub gas: String,
    pub gasPrice: String,
    pub hash: String,
    pub input: String, // or data
    pub nonce: String,
    pub r: String,
    pub s: String,
    pub to: Option<String>,
    pub transactionIndex: String,
    #[serde(rename = "type")]
    pub txType: String,
    pub v: String,
    pub value: String,
    pub typed_tx: TypedTransaction,
}

impl Transaction {
    pub fn rlp_serialize_tx(
        &mut self,
        chain_id: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // To have this work, we need to RLP encode tx params.
        // 0) `nonce`
        // 1) `gas_price`
        // 2) `gas_limit`
        // 3) `to`
        // 4) `value`
        // 5) `data`
        // 6) `v`
        // 7) `r`
        // 8) `s`

        // This way of dealing with the borrow checker is bad byt fuck it we ball

        self.typed_tx.set_to(self.to.clone().expect("REASON").as_str());

        let nonce: U256 = Cow::Borrowed(&self.nonce).parse()?;
        self.typed_tx.set_nonce(nonce);

        let value: U256 = Cow::Borrowed(self.value.as_str()).parse()?;
        self.typed_tx.set_value(value);

        let gas_price: U256 = Cow::Borrowed(&self.gasPrice).parse()?;
        self.typed_tx.set_gas_price(gas_price);

        let gas: U256 = Cow::Borrowed(&self.gas).parse()?;
        self.typed_tx.set_gas(gas);
        self.typed_tx.set_chain_id(chain_id);
        // We need to convert `self.input` to Bytes first to set the data
        let input = hex::decode(self.input.as_str())?;
        self.typed_tx.set_data(input.into());

        // convert r and s to U256
        // convert v to U64
        let r: U256 = self.r.parse()?;
        let s: U256 = self.s.parse()?;
        let v: u64 = self.v.parse()?;

        // create a new use ethers::types::Signature with the r, s, and v values
        let sig: Signature = Signature {
            r, // as U256
            s, // as U256
            v, // as U64
        };

        let encoded = self.typed_tx.rlp_signed(&sig);
        Ok(hex::encode(encoded))
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
