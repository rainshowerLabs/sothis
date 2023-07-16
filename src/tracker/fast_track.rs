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
    path: String,
    filename: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();

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

		if query_interval.is_some() {
			current_block += query_interval.unwrap();
		} else {
			current_block += 1;
		}
	
	}
	
	let json = serde_json::to_string(&storage)?;

	// Set the filename to `address{contract_address}-slot-{storage_slot}-timestamp-{unix_timestamp} if its the default one
	let filename = if filename == "" {
		let timestamp = get_latest_unix_timestamp();
		format!("address-{}-slot-{}-timestamp-{}.json", contract_address, storage_slot, timestamp)
	} else {
		filename
	};

	let path = format!("{}/{}", path, filename);
	println!("\nWriting to file: {}", path);
	fs::write(path, json)?;

	Ok(())
}
