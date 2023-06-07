use std::thread::sleep;

use tokio::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest::Client;
use rlp::RlpStream;

use crate::hex_to_decimal;
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

    // Sends raw transaction
    pub async fn send_raw_transaction(
        &self,
        tx: Transaction,
    ) -> Result<String, RequestError> {
        // To have this work, we need to RLP encode tx params.
        // 0) `nonce`
        // 1) `gas_price`
        // 2) `gas_limit`
        // 3) `to`
        // 4) `value`
        // 5) `data`
        // 6) `v`
        // 7) `r`
        // 8) `s`

        let mut stream = RlpStream::new();
        stream.begin_unbounded_list();
        stream
            .append(&hex_to_decimal(&tx.nonce)?)
            .append(&hex_to_decimal(&tx.gasPrice)?)
            .append(&hex_to_decimal(&tx.gas)?)
            .append(&tx.to)
            .append(&hex_to_decimal(&tx.value)?)
            .append(&tx.value)
            .append(&tx.v)
            .append(&tx.r)
            .append(&tx.s)
        .finalize_unbounded_list();

        let stream = stream.out();
        let params = json!(*stream);

        println!("params: {:#?}", params);

        Ok(self.send_request("eth_sendRawTransaction", params).await?)
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
    
    // Gets hardhat mining mode. We use this to check if our node is HH or anvil.
    pub async fn hardhat_get_automine(&self) -> Result<String, RequestError> {
        Ok(self.send_request("hardhat_getAutomine", serde_json::Value::Null).await?)
    }

    /* 
     * Subscriptions
     */

    // Listen for new blocks, return latest blocknumber on new block.
    pub async fn listen_for_blocks(&self) -> Result<String, RequestError> {
        // theres a million ways to do it than this but i couldnt be bothered
        let blocknumber = self.block_number().await?;
        let mut new_blocknumber = blocknumber.clone();
        println!("Listening for new blocks from block {}...", hex_to_decimal(&blocknumber).unwrap());

        while blocknumber == new_blocknumber {
            // sleep for 1 second
            sleep(Duration::from_millis(1000));
            new_blocknumber = self.block_number().await?
        }

        Ok(new_blocknumber)
    }

    /* 
     * Helper functions
     */

    // This just kinda abstracts the check of if its hardhat.
    pub async fn is_hardhat(&self) -> bool {
        // call hardhat_get_automine and if we get a response, assume we are on hardhat
        match self.hardhat_get_automine().await {
            Ok(_) => return true,
            Err(_) => return false,
        };
    }

}
