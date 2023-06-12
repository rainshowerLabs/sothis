use serde::{Deserialize, Serialize};
use ethers::utils::hex;

use std::str::FromStr;
use std::borrow::Cow;

use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::Signature;
use ethers::types::U256;

use crate::hex_to_decimal;

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
#[allow(non_snake_case)]
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

        // features set to legacy, this is a legacy tx
        let mut typed_tx: TypedTransaction = Default::default();

        // If to doesnt contain a value, set it
        match self.to {
            Some(_) => {
                typed_tx.set_to(self.to.clone().expect("REASON"));
            },
            None => (),
        };

        // This way of dealing with the borrow checker is probably not good but fuck it we ball

        let nonce: U256 = Cow::Borrowed(&self.nonce).parse()?;
        typed_tx.set_nonce(nonce);

        let value: U256 = Cow::Borrowed(&self.value.as_str()).parse()?;
        typed_tx.set_value(value);

        let gas_price: U256 = Cow::Borrowed(&self.gasPrice).parse()?;
        typed_tx.set_gas_price(gas_price);

        let gas: U256 = Cow::Borrowed(&self.gas).parse()?;
        typed_tx.set_gas(gas);

        typed_tx.set_chain_id(chain_id);

        // We need to convert `self.input` to Bytes first to set the data

        // Remove 0x prefix from input if present
        let input = self.input.trim_start_matches("0x");

        let input = hex::decode(input)?;
        let input: ethers::types::Bytes = input.into();
        typed_tx.set_data(input);

        // convert r and s to U256
        // convert v to U64
        // r, s and v are str's. it doesnt matter too much performance wise that we
        // are converting it here since we are only using it here
        let r: U256 = U256::from_str(&self.r)?;
        let s: U256 = U256::from_str(&self.s)?;
        let v: u64 = hex_to_decimal(&self.v)?;

        // create a new use ethers::types::Signature with the r, s, and v values
        let sig: Signature = Signature {
            r, // as U256
            s, // as U256
            v, // as U64
        };

        let encoded = typed_tx.rlp_signed(&sig);
        println!("ENCODED: {:?}", hex::encode(typed_tx.rlp_signed(&sig)));
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
