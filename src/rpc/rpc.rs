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

        println!("Sending request: {}", request);

        let response = self
            .client
            .post(&self.url)
            .json(&request)
            .send()
            .await?
            .json::<JsonRpcResponse>()
            .await?;

        Ok(response.result.to_string())
    }

    /*
    //////////////////////////////////////////////////////////////
                            JSON-RPC METHODS
    //////////////////////////////////////////////////////////////
    */

    pub async fn block_number(&self) -> Result<String, reqwest::Error> {
        // Empty `Value` to statisfy the call params


        let number = self.send_request("eth_blockNumber", serde_json::Value::Null).await?;
        let return_number = format_hex(&number);
        Ok(return_number.to_string())
    }

    pub async fn chain_id(&self) -> Result<String, reqwest::Error> {
        let number = self.send_request("eth_chainId", serde_json::Value::Null).await?;
        let return_number = format_hex(&number);
        Ok(return_number.to_string())
    }

    pub async fn get_block_by_number(
        &self,
        block_number: String,
    ) -> Result<String, reqwest::Error> {

        let params = json!([block_number, true]);

        self.send_request("eth_getBlockByNumber", params).await
    }

    pub async fn get_transaction_by_hash( &self, tx_hash: String) -> Result<String, reqwest::Error> {
        self.send_request("eth_getTransactionByHash", serde_json::Value::Null)
            .await
    }

    pub async fn evm_set_automine(&self, mode: String) -> Result<String, reqwest::Error> {
        self.send_request("evm_setAutomine", serde_json::Value::Null).await
    }

    pub async fn evm_mine(&self) -> Result<String, reqwest::Error> {
        self.send_request("evm_mine", serde_json::Value::Null).await
    }

    pub async fn evm_set_interval_mining(
        &self,
        interval: String,
    ) -> Result<String, reqwest::Error> {
        self.send_request("evm_setIntervalMining", serde_json::Value::Null).await
    }

    pub async fn evm_set_next_block_timestamp(
        &self,
        timestamp: String,
    ) -> Result<String, reqwest::Error> {
        self.send_request("evm_setNextBlockTimestamp", serde_json::Value::Null)
            .await
    }
}
