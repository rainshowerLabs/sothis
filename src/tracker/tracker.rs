use crate::APP_CONFIG;
use crate::RpcConnection;
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
    terminal_block: String,
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

	// Release the lock immediately since we're gonna use it later
    let block_time;
    let path;
    let filename;
    {
        let app_config = APP_CONFIG.lock()?;
        block_time = app_config.block_listen_time;
        path = app_config.path.clone();
        filename = app_config.filename.clone();
    }

    let mut block_number = source_rpc.block_number().await?;
	loop {
        if interrupted.load(Ordering::SeqCst) || block_number == terminal_block {
            break;
        }

		let block_number_u256: U256 = block_number.parse()?;
		let latest_slot = source_rpc.get_storage_at(contract_address.clone(), storage_slot.clone()).await?;

		let slot = StateChange {
			block_number: block_number_u256,
			value: latest_slot,
		};

		if storage.state_changes.last().map(|change| change.value != slot.value).unwrap_or(true) {
			println!("New storage slot value: {:?}", &slot.value);
			storage.state_changes.push(slot);
		}

		block_number = source_rpc.listen_for_blocks(block_time).await?;
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
	println!("Writing to file: {}", path);
	fs::write(path, json)?;

	Ok(())
}
