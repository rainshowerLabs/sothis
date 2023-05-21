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
#[warn(dead_code)]
struct JsonRpcResponse {
    jsonrpc: String,
    result: Value,
    id: u32,
}

pub async fn post(url: String, method: String, param: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method,
        params: json!(param),
        id: 1,
    };
    
    let response = client
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
pub async fn get_block_number(historical_rpc: String) -> Result<String, Box<dyn std::error::Error>> {
    post(historical_rpc, "eth_blockNumber".to_string(), "".to_string()).await
}

// Get block by number
pub async fn get_block_by_number(historical_rpc: String, block_number: String) -> Result<String, Box<dyn std::error::Error>> {
    post(historical_rpc, "eth_getBlockByNumber".to_string(), format!("\"{}\", true", block_number)).await
}
