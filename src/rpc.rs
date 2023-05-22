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

struct RpcConnection {
    client: Client,
    url: String
}

#[async_trait]
trait RpcRequests {
    fn new(url: String) -> Self;
    async fn post(&self, url: String, method: String, param: String) -> Result<String, Box<dyn std::error::Error>>;
    async fn block_number(&self) -> Result<String, Box<dyn std::error::Error>>;
    async fn get_block_by_number(&self, block_number: String) -> Result<String, Box<dyn std::error::Error>>;
}

#[async_trait]
impl RpcRequests for RpcConnection {
    fn new(url: String) -> Self {
        Self {
            client: Client::new(),
            url: url
        }
    }

    async fn post(&self, url: String, method: String, param: String) -> Result<String, Box<dyn std::error::Error>> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method,
            params: json!(param),
            id: 1,
        };
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json::<JsonRpcResponse>()
            .await?;
        
        let block_number_hex = response.result;
        
        Ok(block_number_hex.to_string())
    }

    // Gets blocknumber
    async fn block_number(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.post(self.url.clone(), "eth_blockNumber".to_string(), "".to_string()).await
    }

    // Get block by number
    async fn get_block_by_number(&self, block_number: String) -> Result<String, Box<dyn std::error::Error>> {
        self.post(self.url.clone(), "eth_getBlockByNumber".to_string(), format!("\"{}\", true", block_number)).await
    }
}
