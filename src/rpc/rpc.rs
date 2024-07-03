use std::thread::sleep;
use std::time::Instant;
use url::Url;

use ethers::types::U256;
use reqwest::Client;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::{
    json,
    Value,
};
use tokio::time::Duration;

use super::error::*;
use super::format::format_hex;
use super::types::*;
use crate::hex_to_decimal;

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

#[derive(Clone)]
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
            url: Url::parse(&url).expect("Your url is invalid!").into(),
        }
    }

    // Generic fn to send rpc
    async fn send_request(&self, method: &str, mut params: Value) -> Result<String, RequestError> {
        // We do this because eth rpc cries if param is empty

        if params.is_null() {
            params = json!([]);
        }

        let request: Value = json!({
            "method": method.to_string(),
            "params": params,
            "id": 1,
            "jsonrpc": "2.0".to_string(),
        });

        // #[cfg(debug_assertions)] {
        //     println!("Sending request: {}", request.clone());
        // }

        let response = match self.client.post(&self.url).json(&request).send().await {
            Ok(response) => response,
            Err(err) => return Err(RequestError::JsonSerializationFailed(err.to_string())),
        };

        let response: serde_json::Value = match response.json().await {
            Ok(response) => response,
            Err(err) => return Err(RequestError::JsonDeserializationFailed(err.to_string())),
        };

        let response = match serde_json::from_value::<JsonRpcResponse>(response.clone()) {
            Ok(response) => response,
            Err(_) => {
                // If we cannot get the value here, deserialize as an error and get the response message
                let err = &response["error"]["message"];
                return Err(RequestError::RequestFailed(err.to_string()));
            }
        };

        Ok(response.result.to_string())
    }

    /*
     * JSON-RPC methods
     */

    // Gets current block_number.
    pub async fn block_number(&self) -> Result<String, RequestError> {
        let number = self
            .send_request("eth_blockNumber", serde_json::Value::Null)
            .await?;
        let return_number = format_hex(&number);
        Ok(return_number.to_string())
    }

    // Gets current chain_id.
    pub async fn chain_id(&self) -> Result<String, RequestError> {
        let number = self
            .send_request("eth_chainId", serde_json::Value::Null)
            .await?;
        let return_number = format_hex(&number);
        Ok(return_number.to_string())
    }

    // Gets block info and hashes by block number.
    pub async fn get_block_by_number(&self, block_number: String) -> Result<String, RequestError> {
        let params = json!([block_number, true]);
        self.send_request("eth_getBlockByNumber", params).await
    }

    // Gets storage at address and slot for the latest block
    pub async fn get_storage_at(
        &self,
        address: String,
        slot: U256,
    ) -> Result<String, RequestError> {
        let params = json!([address, slot, "latest"]);
        let result = self.send_request("eth_getStorageAt", params).await?;

        Ok(result.trim_matches('\"').to_string())
    }

    // Gets storage at address and slot for a block specified in th argument
    pub async fn get_storage_at_block(
        &self,
        address: String,
        slot: U256,
        block: String,
    ) -> Result<String, RequestError> {
        let params = json!([address, slot, block]);
        let result = self.send_request("eth_getStorageAt", params).await?;

        Ok(result.trim_matches('\"').to_string())
    }

    // Gets transaction by hash (duh).
    pub async fn get_transaction_by_hash(&self, tx_hash: String) -> Result<String, RequestError> {
        let params = json!([tx_hash]);
        self.send_request("eth_getTransactionByHash", params).await
    }

    // Sends raw transaction
    pub async fn send_raw_transaction(
        &self,
        tx: Transaction,
        chain_id: u64,
    ) -> Result<String, RequestError> {
        let tx = tx.clone();

        let params = tx.rlp_serialize_tx(chain_id)?;
        let params = json!([params]);

        self.send_request("eth_sendRawTransaction", params).await
    }

    // Sends raw transaction
    pub async fn call(&self, tx: CallParams, block_number: String) -> Result<String, RequestError> {
        // TODO: maybe value?
        let params = json!([tx, block_number]);
        let result = self.send_request("eth_call", params).await?;
        Ok(result.trim_matches('\"').to_string())
    }

    /*
     * hardhat/anvil specific RPC
     */

    // Send tx without checking signature
    pub async fn send_unsigned_transaction(
        &self,
        tx: Transaction,
        chain_id: u64,
    ) -> Result<String, RequestError> {
        // Put the relevant values of `Transaction` into `TransactionParams`
        let tx = TransactionParams {
            from: tx.from,
            to: tx.to,
            gas: tx.gas,
            gasPrice: tx.gasPrice,
            value: tx.value,
            data: tx.input,
            nonce: Some(tx.nonce),
            chainId: Some(chain_id.to_string()),
        };

        let params = serde_json::to_value(vec![tx]).unwrap(); // Convert the TransactionParams to a single-element array
        self.send_request("eth_sendUnsignedTransaction", params)
            .await
    }

    // Turn automining on/off. If on, mines on every tx.
    pub async fn evm_set_automine(&self, mode: bool) -> Result<String, RequestError> {
        let params = json!([mode]);
        self.send_request("evm_setAutomine", params).await
    }

    // Mines a block.
    pub async fn evm_mine(&self) -> Result<String, RequestError> {
        self.send_request("evm_mine", serde_json::Value::Null).await
    }

    // Set the interval at which we mine blocks in ms.
    pub async fn evm_set_interval_mining(&self, interval: u64) -> Result<String, RequestError> {
        let params = json!([interval]);
        self.send_request("evm_setIntervalMining", params).await
    }

    // Set the next block's timestamp.
    pub async fn evm_set_next_block_timestamp(
        &self,
        timestamp: u64,
    ) -> Result<String, RequestError> {
        let params = json!([timestamp]);
        self.send_request("evm_setNextBlockTimestamp", params).await
    }

    // Gets hardhat mining mode. We use this to check if our node is HH or anvil.
    pub async fn hardhat_get_automine(&self) -> Result<String, RequestError> {
        self.send_request("hardhat_getAutomine", serde_json::Value::Null)
            .await
    }

    /*
     * Subscriptions
     */

    // Listen for new blocks, return latest blocknumber on new block.
    pub async fn listen_for_blocks(&self, time: u64) -> Result<String, RequestError> {
        // theres a million ways to do it than this but i couldnt be bothered
        let blocknumber = self.block_number().await?;
        let mut new_blocknumber = blocknumber.clone();
        println!(
            "Listening for new blocks from block {}...",
            hex_to_decimal(&blocknumber).unwrap()
        );

        // Start timer for the *heartbeat*
        let mut start_time = Instant::now();

        while blocknumber == new_blocknumber {
            // sleep for set duration
            sleep(Duration::from_millis(time));

            // Add this as a *heartbeat* so users are less confused if nothing is happening
            let elapsed_time = start_time.elapsed();

            if elapsed_time >= Duration::from_secs(20) {
                println!("!!! \x1b[93mNo new blocks have been detected in 20 seconds! Check your node(s)\x1b[0m !!!");
                println!("If your node is stuck at `evm_mine` this means its querrying state needed to replay.");
                println!("Still listening...");
                start_time = Instant::now();
            }

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
        self.hardhat_get_automine().await.is_ok()
    }
}
