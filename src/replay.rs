use clap::{arg, Command};

use crate::rpc::*;

// Gets blocknumber
pub async function get_block_number(historical_rpc: String) -> Result<(), Box<dyn std::error::Error>> {
	post(historical_rpc, "eth_blockNumber".to_string(), "".to_string()).await?;
}
