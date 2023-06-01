use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::format::format_hex;
use super::error::*;
use super::types::*;

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

        // #[cfg(debug_assertions)] {
        //     println!("Sending request: {}", request.clone());
        // }

        let response = match self.client.post(&self.url).json(&request).send().await {
            Ok(response) => response,
            Err(err) => return Err(RequestError::RequestFailed(err.to_string())),
        };

        let response: serde_json::Value = match response.json().await {
            Ok(response) => response,
            Err(err) => return Err(RequestError::JsonDeserializationFailed(err.to_string())),
        };

        let response = match serde_json::from_value::<JsonRpcResponse>(response) {
            Ok(response) => response,
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

    /* 
     * Subscriptions
     */

    // Listen for new blocks, return latest blocknumber on new block.
    pub async fn listen_for_blocks(&self) -> Result<u64, RequestError> {
        let latest_block_number = Arc::new(Mutex::new(0u64));
        loop {
            // Send an eth_blockNumber JSON-RPC request to get the latest block number
            let response = self.client
                .post(self.url.clone())
                .json(&json!({
                    "jsonrpc": "2.0",
                    "method": "eth_blockNumber",
                    "params": [],
                    "id": 1
                }))
                .send()
                .await
                .unwrap();

            // Parse the JSON response
            let response_json: serde_json::Value = response.json().await.unwrap();

            // Extract the latest block number from the response
            let block_number_hex = response_json["result"].as_str().unwrap();
            let block_number = u64::from_str_radix(block_number_hex.trim_start_matches("0x"), 16).unwrap();

            // Check if the latest block number has changed
            let mut latest_block_number = latest_block_number.lock().await;
            if block_number != *latest_block_number {
                // Update the latest block number in the shared state
                *latest_block_number = block_number;
                return Ok(0u64);
            }

            // Delay for a certain period before checking the latest block number again
            tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
        }
    }
}
