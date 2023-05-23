use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest::Client;

#[derive(Debug, Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: Value,
    id: u32,
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
    url: String
}

#[allow(dead_code)]
impl RpcConnection {
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }

    async fn send_request(&self, url: &str, method: &str, param: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params: json!(param),
            id: 1,
        };

        let response = self.client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<JsonRpcResponse>()
            .await?;
        
        let block_number_hex = response.result;
        
        Ok(block_number_hex.to_string())
    }

    pub async fn block_number(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.send_request(&self.url, "eth_blockNumber", "").await
    }

    pub async fn get_block_by_number(&self, block_number: String) -> Result<String, Box<dyn std::error::Error>> {
        self.send_request(&self.url, "eth_getBlockByNumber", &format!("\"{}\", true", block_number)).await
    }
    
    pub async fn get_transaction_by_hash(&self, transaction_hash: String) -> Result<String, Box<dyn std::error::Error>> {
        self.send_request(&self.url, "eth_getTransactionByHash", &transaction_hash).await
    }

    pub async fn evm_set_automine(&self, mode: String) -> Result<String, Box<dyn std::error::Error>> {
        self.send_request(&self.url, "evm_setAutomine", &mode).await
    }

    pub async fn evm_mine(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.send_request(&self.url, "evm_mine", "").await
    }

    pub async fn evm_set_interval_mining(&self, interval: String) -> Result<String, Box<dyn std::error::Error>> {
        self.send_request(&self.url, "evm_setIntervalMining", &interval).await
    }

    pub async fn evm_set_next_block_timestamp(&self, timestamp: String) -> Result<String, Box<dyn std::error::Error>> {
        self.send_request(&self.url, "evm_setNextBlockTimestamp", &timestamp).await
    }
}
