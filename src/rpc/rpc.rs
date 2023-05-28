use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest::Client;

use super::format::format_hex;

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
    ) -> Result<String, reqwest::Error> {
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

        #[cfg(debug_assertions)]
        {
            println!("Sending request: {}", request.clone());
        }

        let response = self
            .client
            .post(&self.url)
            .json(&request)
            .send()
            .await?
            .json::<JsonRpcResponse>()
            .await?;

        #[cfg(debug_assertions)]
        {
            // we need to clone the response here
            println!("Received response: {:?}", &response);
        
        }

        Ok(response.result.to_string())
    }

    /* 
     * JSON-RPC methods
     */

    // Gets current block_number.
    pub async fn block_number(&self) -> Result<String, reqwest::Error> {
        let number = self.send_request("eth_blockNumber", serde_json::Value::Null).await?;
        let return_number = format_hex(&number);
        Ok(return_number.to_string())
    }

    // Gets current chain_id.
    pub async fn chain_id(&self) -> Result<String, reqwest::Error> {
        let number = self.send_request("eth_chainId", serde_json::Value::Null).await?;
        let return_number = format_hex(&number);
        Ok(return_number.to_string())
    }

    // Gets block info and hashes by block number.
    pub async fn get_block_by_number(
        &self,
        block_number: String,
    ) -> Result<String, reqwest::Error> {
        let params = json!([block_number, true]);
        self.send_request("eth_getBlockByNumber", params).await
    }

    // Gets transaction by hash (duh).
    pub async fn get_transaction_by_hash(
        &self,
        tx_hash: String,
    ) -> Result<String, reqwest::Error> {
        let params = json!([tx_hash]);
        self.send_request("eth_getTransactionByHash", params).await
    }

    // Send transaction
    pub async fn send_transaction(
        &self,
        tx: String,
    ) -> Result<String, reqwest::Error> {
        let params = json!([tx]);
        self.send_request("eth_sendTransaction", params).await
    }

    /* 
     * hardhat/anvil specific RPC
     */

    // Turn automining on/off. If on, mines on every tx.
    pub async fn evm_set_automine(
        &self,
        mode: bool,
    ) -> Result<String, reqwest::Error> {
        let params = json!([mode]);
        self.send_request("evm_setAutomine", params).await
    }

    // Mines a block.
    pub async fn evm_mine(&self) -> Result<String, reqwest::Error> {
        self.send_request("evm_mine", serde_json::Value::Null).await
    }

    // Set the interval at which we mine blocks in ms.
    pub async fn evm_set_interval_mining(
        &self,
        interval: u64,
    ) -> Result<String, reqwest::Error> {
        let params = json!([interval]);
        self.send_request("evm_setIntervalMining", params).await
    }

    // Set the next block's timestamp.
    pub async fn evm_set_next_block_timestamp(
        &self,
        timestamp: u64,
    ) -> Result<String, reqwest::Error> {
        let params = json!([timestamp]);
        self.send_request("evm_setNextBlockTimestamp", params).await
    }
}
