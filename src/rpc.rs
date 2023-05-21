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

pub async fn post(url: String, method: String, param: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    // box leak this bih
    //let method = Box::leak(method.into_boxed_str());

    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: method,
        params: json!(param),
        id: 1,
    };
    
    let response = client
        .post(url)
        .json(&request)
        .send()
        .await?
        .json::<JsonRpcResponse>()
        .await?;
    
    println!("{:?}", response.result);
    
    Ok(())
}