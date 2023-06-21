use crate::hex_to_decimal;
use crate::APP_CONFIG;
use crate::RpcConnection;
use crate::tracker::types::*;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use ctrlc;
use ethers::types::U256;

// We listen for new blocks and get the storage slot value if changed.
pub async fn track_state(
    source_rpc: RpcConnection,
    storage_slot: U256,
    contract_address: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();

    ctrlc::set_handler(move || {
        interrupted_clone.store(true, Ordering::SeqCst);
    })?;

	let mut storage = StateChangeList {
		storage_slot: storage_slot,
		state_changes: Vec::new(),
	};

	// Release the lock immediately since we're gonna use it later
    let block_time;
    {
        let app_config = APP_CONFIG.lock()?;
        block_time = app_config.block_listen_time;
    }

	loop {
        if interrupted.load(Ordering::SeqCst) {
            break;
        }

		let block_number = source_rpc.listen_for_blocks(block_time).await?;
		let block_number = hex_to_decimal(&block_number)?; // FIXME: this returns a u64, change this
		let latest_slot = source_rpc.get_storage_at(contract_address.clone(), storage_slot.clone()).await?;

		let slot = StateChange {
			block_number: block_number.into(),
			value: latest_slot,
		};

		if storage.state_changes.last().map(|change| change.value != slot.value).unwrap_or(true) {
			println!("New storage slot value: {:?}", &slot.value);
			storage.state_changes.push(slot);
		}
	}
	let json = serde_json::to_string(&storage)?;
	println!("Serialized storage: {}", json);

	Ok(())
}
