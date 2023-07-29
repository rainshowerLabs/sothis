use crate::RpcConnection;
use crate::rpc::format::{hex_to_decimal, decimal_to_hex};
use crate::tracker::types::*;
use crate::tracker::time::get_latest_unix_timestamp;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::fs;

use ctrlc;
use ethers::types::U256;

// We querry historical storage from a node instead of waiting for new blocks.
pub async fn fast_track_state(
    source_rpc: RpcConnection,
    storage_slot: U256,
    contract_address: String,
    terminal_block: Option<u64>,
    origin_block: u64,
    query_interval: Option<u64>,
    as_dec: bool,
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
    	println!("!!! \x1b[93mWARNING:\x1b[0m Query interval is set, sothis will not have the full context of the storage slot changes !!!");
    	interval = query_interval.unwrap();
	}

    ctrlc::set_handler(move || {
        interrupted_clone.store(true, Ordering::SeqCst);
    })?;

	let mut storage = StateChangeList {
		address: contract_address.clone(),
		storage_slot: storage_slot,
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

		let latest_slot = source_rpc.get_storage_at_block(contract_address.clone(), storage_slot.clone(), decimal_to_hex(current_block)).await?;
		let slot = StateChange {
			block_number: current_block.into(),
			value: latest_slot,
		};

		if storage.state_changes.last().map(|change| change.value != slot.value).unwrap_or(true) {
			println!("New storage slot value at block {}: {:?}", slot.block_number, &slot.value);
			storage.state_changes.push(slot);
		}

		current_block += interval;
	}
	
	// Set the filename to `address{contract_address}-slot-{storage_slot}-timestamp-{unix_timestamp} if its the default one
	// We also check if we should serialize it as csv
	let filename = match filename.as_str() {
		"" => {
			let timestamp = get_latest_unix_timestamp();
			println!("No filename specified, using default and formatting as JSON");
			format!("address-{}-slot-{}-timestamp-{}.json", contract_address, storage_slot, timestamp)
		},
		_ => {
			filename
		},
	};

	// This is a mid solution
	// as_dec, is_csv
	let mut is_csv = false;
	if filename.contains(".csv") {
		is_csv = true;
	}

	let json;
	match (as_dec, is_csv) {
		(false, false) => json = storage.serialize_json()?,
		(true, false) => json = storage.serialize_json_dec()?,
		(false, true) => json = storage.serialize_csv(),
		(true, true) => json = storage.serialize_csv_dec(),
	};

	let path = format!("{}/{}", path, filename);
	println!("\nWriting to file: {}", path);
	fs::write(path, json)?;

	Ok(())
}
