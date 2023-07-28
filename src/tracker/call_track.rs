use crate::RpcConnection;
use crate::rpc::format::{hex_to_decimal, decimal_to_hex};
use crate::tracker::types::*;
use crate::tracker::time::get_latest_unix_timestamp;
use crate::rpc::types::TransactionParams;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::fs;

use ctrlc;

pub async fn call_track(
    source_rpc: RpcConnection,
    calldata: String,
    contract_address: String,
    terminal_block: Option<u64>,
    origin_block: u64,
    query_interval: Option<u64>,
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
		calldata: calldata,
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

        // TODO: the following
        let tx = TransactionParams { 
        	from: (),
        	to: (),
        	value: (),
        	gas: (),
        	gasPrice: (),
        	data: (),
        	chainId: None,
			nonce: None,

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
	
	// Set the filename to `address{contract_address}-slot-{storage_slot}-timestamp-{unix_timestamp} if its the default one
	// We also check if we should serialize it as csv
	let json;
	let filename = match filename.as_str() {
		"" => {
			let timestamp = get_latest_unix_timestamp();
			println!("No filename specified, using default and formatting as JSON");
			json = storage.serialize_json()?;
			format!("address-{}-calldata-{}-timestamp-{}.json", contract_address, calldata, timestamp)
		},
		filename if filename.contains(".csv") => {
			println!("Formatting as CSV...");
			json = storage.serialize_csv();
			filename.to_string()
		},
		_ => {
			json = storage.serialize_json()?;
			filename
		},
	};

	let path = format!("{}/{}", path, filename);
	println!("\nWriting to file: {}", path);
	fs::write(path, json)?;

	Ok(())
}