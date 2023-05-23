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

#[allow(dead_code, unused_variables)]
impl RpcConnection {
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }

    async fn send_request(
        &self,
        method: &str,
        param: &str,
    ) -> Result<String, reqwest::Error> {
        let request = JsonRpcRequest {
            method: method.to_string(),
            params: json!([param]),
            id: 1,
            jsonrpc: "2.0".to_string(),
        };

        println!("Sending request: {:?}", request);

        

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

    pub async fn block_number(&self) -> Result<String, reqwest::Error> {
        let number = self.send_request("eth_blockNumber", "").await?;
        let return_number = format_hex(&number);
        Ok(return_number.to_string())
    }

    pub async fn chain_id(&self) -> Result<String, reqwest::Error> {
        let number = self.send_request("eth_chainId", "").await?;
        let return_number = format_hex(&number);
        Ok(return_number.to_string())
    }

    pub async fn get_block_by_number(
        &self,
        block_number: String,
    ) -> Result<String, reqwest::Error> {
        self.send_request("eth_getBlockByNumber", &format!("\"{}\", true", block_number)).await
    }

    pub async fn get_transaction_by_hash( &self, tx_hash: String) -> Result<String, reqwest::Error> {
        self.send_request("eth_getTransactionByHash", &tx_hash)
            .await
    }

    pub async fn evm_set_automine(&self, mode: String) -> Result<String, reqwest::Error> {
        self.send_request("evm_setAutomine", &mode).await
    }

    pub async fn evm_mine(&self) -> Result<String, reqwest::Error> {
        self.send_request("evm_mine", "").await
    }

    pub async fn evm_set_interval_mining(
        &self,
        interval: String,
    ) -> Result<String, reqwest::Error> {
        self.send_request("evm_setIntervalMining", &interval).await
    }

    pub async fn evm_set_next_block_timestamp(
        &self,
        timestamp: String,
    ) -> Result<String, reqwest::Error> {
        self.send_request("evm_setNextBlockTimestamp", &timestamp)
            .await
    }
}
