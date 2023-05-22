use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest::Client;
use async_trait::async_trait;

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

#[async_trait]
trait RpcRequests {
    async fn send_request(&self, url: &str, method: &str, param: &str) -> Result<String, Box<dyn std::error::Error>>;
    async fn block_number(&self) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_block_by_number(&self, block_number: &str) -> Result<String, Box<dyn std::error::Error>>;
}

#[allow(unreachable_code, unused_variables)]
#[async_trait]
impl RpcRequests for RpcConnection {
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

    async fn block_number(&self) -> Result<String, Box<dyn std::error::Error>> {
        !unreachable!();
    }

    async fn get_block_by_number(&self, block_number: &str) -> Result<String, Box<dyn std::error::Error>> {
        !unreachable!();
    }
}

#[allow(dead_code)]
impl RpcConnection {
    pub fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            url,
        }
    }

    pub async fn block_number(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.send_request(&self.url, "eth_blockNumber", "").await
    }

    pub async fn get_block_by_number(&self, block_number: String) -> Result<String, Box<dyn std::error::Error>> {
        self.send_request(&self.url, "eth_getBlockByNumber", &format!("\"{}\", true", block_number)).await
    }
}
