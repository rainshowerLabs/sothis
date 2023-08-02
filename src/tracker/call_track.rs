use crate::RpcConnection;
use crate::rpc::format::{hex_to_decimal, decimal_to_hex};
use crate::tracker::types::*;
use crate::rpc::types::CallParams;
use serde_json::Value;
use crate::tracker::common::set_filename_and_serialize;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use ctrlc;

pub async fn call_track(
    source_rpc: RpcConnection,
    calldata: String,
    contract_address: String,
    terminal_block: Option<u64>,
    origin_block: u64,
    query_interval: Option<u64>,
    decimal: bool,
    path: String,
    filename: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();
    
    // Set how much we're tracking by
    // Default is that we are checking every block for state changes
    let mut interval = 1;

    // Print warning that sothis does not have the full context
    if query_interval.is_some() {
    	println!("!!! \x1b[93mWARNING:\x1b[0m Query interval is set, sothis will not have the full context of the eth_calls !!!");
    	interval = query_interval.unwrap();
	}

    ctrlc::set_handler(move || {
        interrupted_clone.store(true, Ordering::SeqCst);
    })?;

	let mut storage = CallChangeList {
		address: contract_address.clone(),
		calldata: calldata.clone(),
		state_changes: Vec::new(),
	};

	let terminal_block = match terminal_block.is_some() {
		true => terminal_block.unwrap(),
		false => {
			let a = hex_to_decimal(&source_rpc.block_number().await?)?;
			println!("No terminal block set, setting terminal block to current head: {}", a);
			a
		},
	};

	// Error out if the origin block is >= than the terminal
	if origin_block >= terminal_block {
		return Err("Origin block cannot be higher than the terminal block".into());
	}

	let mut current_block = origin_block;
	while current_block < terminal_block {
        if interrupted.load(Ordering::SeqCst) {
            break;
        }

        let tx = CallParams { 
        	from: Value::Null,
            to: contract_address.clone(),
            data: calldata.clone(),
        };

		let latest_call = source_rpc.call(tx, decimal_to_hex(current_block)).await?;
		let slot = StateChange {
			block_number: current_block.into(),
			value: latest_call,
		};

		if storage.state_changes.last().map(|change| change.value != slot.value).unwrap_or(true) {
			println!("New storage slot value at block {}: {:?}", slot.block_number, &slot.value);
			storage.state_changes.push(slot);
		}

		current_block += interval;
	}
	
	set_filename_and_serialize(path, filename, storage, contract_address, "calldata", calldata, decimal)?;

	Ok(())
}