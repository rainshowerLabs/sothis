use ethers::types::transaction::eip2930::AccessList;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::Value;

use ethers::types::{
    Bytes,
    Eip1559TransactionRequest,
    TransactionRequest,
    H160,
};
use ethers::utils::hex;

use std::str::FromStr;

use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::Signature;
use ethers::types::U256;

use crate::hex_to_decimal;

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code, non_snake_case)]
pub struct BlockResult {
    difficulty: Option<String>,
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
    totalDifficulty: Option<String>,
    pub transactions: Vec<Transaction>,
    transactionsRoot: String,
    uncles: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Transaction {
    pub blockHash: String,
    pub blockNumber: String,
    pub hash: String,
    pub accessList: Option<Vec<Value>>,
    pub chainId: Option<String>,
    pub from: String,
    pub gas: String,
    pub gasPrice: String,
    pub input: String, // or data
    pub maxFeePerGas: Option<String>,
    pub maxPriorityFeePerGas: Option<String>,
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
    pub fn rlp_serialize_tx(&self, chain_id: u64) -> Result<String, Box<dyn std::error::Error>> {
        let encoded;
        // if access list exists we need the typed transaction to be an eip1559 one
        if self.maxFeePerGas.is_some() {
            encoded = self.rlp_serialize_eip1559(chain_id)?;
        } else {
            encoded = self.rlp_serialize_legacy(chain_id)?;
        }

        //println!("ENCODED: {:?}", hex::encode(typed_tx.rlp_signed(&sig)));
        Ok(encoded)
    }

    fn rlp_serialize_eip1559(&self, chain_id: u64) -> Result<String, Box<dyn std::error::Error>> {
        let to = match &self.to {
            Some(_) => {
                Some(ethers::types::NameOrAddress::Address(H160::from_str(
                    &self.to.clone().unwrap(),
                )?))
            }
            None => None,
        };

        let transaction = Eip1559TransactionRequest {
            from: Some(H160::from_str(&self.from).unwrap()),
            to,
            gas: Some(U256::from_str(&self.gas)?),
            value: Some(U256::from_str(&self.value)?),
            data: Some(Bytes::from(hex::decode(
                self.input.trim_start_matches("0x"),
            )?)), // ?????
            nonce: Some(U256::from_str(&self.nonce)?),
            access_list: AccessList::default(), // TODO: make this not-default later. its optional so who cares for now
            max_priority_fee_per_gas: Some(U256::from_str(
                &self.maxPriorityFeePerGas.clone().unwrap(),
            )?),
            max_fee_per_gas: Some(U256::from_str(&self.maxFeePerGas.clone().unwrap())?),
            chain_id: Some(chain_id.into()),
        };

        let typed_tx = TypedTransaction::Eip1559(transaction);

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

        Ok(hex::encode(typed_tx.rlp_signed(&sig)))
    }

    fn rlp_serialize_legacy(&self, chain_id: u64) -> Result<String, Box<dyn std::error::Error>> {
        let to = match &self.to {
            Some(_) => {
                Some(ethers::types::NameOrAddress::Address(H160::from_str(
                    &self.to.clone().unwrap(),
                )?))
            }
            None => None,
        };

        let transaction = TransactionRequest {
            from: Some(H160::from_str(&self.from).unwrap()),
            to,
            gas: Some(U256::from_str(&self.gas)?),
            gas_price: Some(U256::from_str(&self.gasPrice)?),
            value: Some(U256::from_str(&self.value)?),
            data: Some(Bytes::from(hex::decode(
                self.input.trim_start_matches("0x"),
            )?)),
            nonce: Some(U256::from_str(&self.nonce)?),
            chain_id: Some(chain_id.into()),
        };

        let typed_tx = TypedTransaction::Legacy(transaction);

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

        let encoded = hex::encode(typed_tx.rlp_signed(&sig));
        // Add 0x prefix to encoded tx
        let encoded = format!("0x{}", encoded);

        Ok(encoded)
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
    pub nonce: Option<String>,
    pub chainId: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CallParams {
    pub from: serde_json::value::Value,
    pub to: String,
    pub data: String,
}
