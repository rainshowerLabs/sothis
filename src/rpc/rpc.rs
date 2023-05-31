use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest::Client;

use super::format::format_hex;
use super::error::*;

#[derive(Debug, Serialize)]
struct JsonRpcRequest {
    method: String,
    params: Value,
    id: u32,
    jsonrpc: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Value,
    id: u32,
}

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
    blockHash: String,
    blockNumber: String,
    pub from: String,
    pub gas: String,
    pub gasPrice: String,
    pub hash: String,
    pub input: String,
    pub nonce: String,
    r: String,
    s: String,
    pub to: Option<String>,
    transactionIndex: String,
    #[serde(rename = "type")]
    txType: String,
    v: String,
    pub value: String,
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

pub struct RpcConnection {
    client: Client,
    url: String,
}

impl Clone for RpcConnection {
    fn clone(&self) -> Self {
        RpcConnection {
            client: self.client.clone(),
            url: self.url.clone(),
        }
    }
}

#[allow(dead_code)]
impl RpcConnection {
    // Create client and set url
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }

    // Generic fn to send rpc
    async fn send_request(
        &self,
        method: &str,
        params: Value,
    ) -> Result<String, RequestError> {
        // We do this because eth rpc cries if param is empty
        let request: Value;
        if params.is_null() {
            request = json!({
                "method": method.to_string(),
                "params": [],
                "id": 1,
                "jsonrpc": "2.0".to_string(),
            });
        } else {
            request = json!({
                "method": method.to_string(),
                "params": params,
                "id": 1,
                "jsonrpc": "2.0".to_string(),
            });
        }

        #[cfg(debug_assertions)] {
            println!("Sending request: {}", request.clone());
        }

        let response = match self.client.post(&self.url).json(&request).send().await {
            Ok(response) => response,
            Err(err) => return Err(RequestError::RequestFailed(err.to_string())),
        };

        let response: serde_json::Value = match response.json().await {
            Ok(json) => json,
            Err(err) => return Err(RequestError::JsonDeserializationFailed(err.to_string())),
        };

        let response = match serde_json::from_value::<JsonRpcResponse>(response) {
            Ok(rpc_response) => rpc_response,
            Err(err) => return Err(RequestError::JsonDeserializationFailed(err.to_string())),
        };

        Ok(response.result.to_string())
    }

    /* 
     * JSON-RPC methods
     */

    // Gets current block_number.
    pub async fn block_number(&self) -> Result<String, RequestError> {
        let number = self.send_request("eth_blockNumber", serde_json::Value::Null).await?;
        let return_number = format_hex(&number);
        Ok(return_number.to_string())
    }

    // Gets current chain_id.
    pub async fn chain_id(&self) -> Result<String, RequestError> {
        let number = self.send_request("eth_chainId", serde_json::Value::Null).await?;
        let return_number = format_hex(&number);
        Ok(return_number.to_string())
    }

    // Gets block info and hashes by block number.
    pub async fn get_block_by_number(
        &self,
        block_number: String,
    ) -> Result<String, RequestError> {
        let params = json!([block_number, true]);
        Ok(self.send_request("eth_getBlockByNumber", params).await?)
    }

    // Gets transaction by hash (duh).
    pub async fn get_transaction_by_hash(
        &self,
        tx_hash: String,
    ) -> Result<String, RequestError> {
        let params = json!([tx_hash]);
        Ok(self.send_request("eth_getTransactionByHash", params).await?)
    }

    // Send transaction
    pub async fn send_transaction(
        &self,
        tx: TransactionParams,
    ) -> Result<String, RequestError> {
        let params = serde_json::to_value(vec![tx]).unwrap();  // Convert the TransactionParams to a single-element array
        Ok(self.send_request("eth_sendTransaction", params).await?)
    }

    /* 
     * hardhat/anvil specific RPC
     */

    // Send tx without checking signature
    pub async fn send_unsigned_transaction(
        &self,
        tx: TransactionParams,
    ) -> Result<String, RequestError> {
        let params = serde_json::to_value(vec![tx]).unwrap();  // Convert the TransactionParams to a single-element array
        Ok(self.send_request("eth_sendUnsignedTransaction", params).await?)
    }

    // Turn automining on/off. If on, mines on every tx.
    pub async fn evm_set_automine(
        &self,
        mode: bool,
    ) -> Result<String, RequestError> {
        let params = json!([mode]);
        Ok(self.send_request("evm_setAutomine", params).await?)
    }

    // Mines a block.
    pub async fn evm_mine(&self) -> Result<String, RequestError> {
        Ok(self.send_request("evm_mine", serde_json::Value::Null).await?)
    }

    // Set the interval at which we mine blocks in ms.
    pub async fn evm_set_interval_mining(
        &self,
        interval: u64,
    ) -> Result<String, RequestError> {
        let params = json!([interval]);
        Ok(self.send_request("evm_setIntervalMining", params).await?)
    }

    // Set the next block's timestamp.
    pub async fn evm_set_next_block_timestamp(
        &self,
        timestamp: u64,
    ) -> Result<String, RequestError> {
        let params = json!([timestamp]);
        Ok(self.send_request("evm_setNextBlockTimestamp", params).await?)
    }
}
