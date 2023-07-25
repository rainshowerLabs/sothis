use crate::RpcConnection;
use crate::rpc::format::hex_to_decimal;
use crate::tracker::types::*;
use crate::tracker::time::get_latest_unix_timestamp;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::fs;

use ctrlc;
use ethers::types::U256;

// We listen for new blocks and get the storage slot value if changed.
pub async fn track_state(
    source_rpc: RpcConnection,
    storage_slot: U256,
    contract_address: String,
    terminal_block: Option<u64>,
    block_listen_time: u64,
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

    let mut block_number = source_rpc.block_number().await?;
	loop {
		// Crazy hamburger check
		let has_reached_terminal_block = terminal_block.as_ref().map(|tb| hex_to_decimal(&block_number).unwrap() >= *tb).unwrap_or(false);
        if interrupted.load(Ordering::SeqCst) || has_reached_terminal_block {
            break;
        }

		let block_number_u256: U256 = block_number.parse()?;
		let latest_slot = source_rpc.get_storage_at(contract_address.clone(), storage_slot.clone()).await?;

		let slot = StateChange {
			block_number: block_number_u256,
			value: latest_slot,
		};

		if storage.state_changes.last().map(|change| change.value != slot.value).unwrap_or(true) {
			println!("New storage slot value at block {}: {:?}", slot.block_number, &slot.value);
			storage.state_changes.push(slot);
		}

		block_number = source_rpc.listen_for_blocks(block_listen_time).await?;
	}

	// Set the filename to `address{contract_address}-slot-{storage_slot}-timestamp-{unix_timestamp} if its the default one
	// We also check if we should serialize it as csv
	let mut is_csv = false;
	println!("{:?}", filename);
	let filename = match filename.as_str() {
		"" => {
			let timestamp = get_latest_unix_timestamp();
			println!("No filename specified, using default and formatting as JSON");
			format!("address-{}-slot-{}-timestamp-{}.json", contract_address, storage_slot, timestamp)
		},
		filename if filename.contains(".csv") => {
			println!("Formatting as CSV");
			is_csv = true;
			filename.to_string()
		},
		_ => filename,
	};

	let json = match is_csv {
		true => storage.serialize_csv(),
		false => storage.serialize_json()?,
	};

	let path = format!("{}/{}", path, filename);
	println!("\nWriting to file: {}", path);
	fs::write(path, json)?;

	Ok(())
}
