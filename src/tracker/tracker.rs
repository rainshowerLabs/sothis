use crate::APP_CONFIG;
use crate::RpcConnection;
use crate::tracker::types::*;

use ethers::types::U256;

// We listen for new blocks and get the storage slot value if changed.
#[allow(unreachable_code)]
pub async fn track_state(
    source_rpc: RpcConnection,
    storage_slot: U256,
    contract_address: String,
) -> Result<(), Box<dyn std::error::Error>> {
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
		let block_number = source_rpc.listen_for_blocks(block_time).await?;
		let latest_slot = source_rpc.get_storage_at(contract_address.clone(), storage_slot.clone()).await?;

		let slot = StateChange {
			block_number: block_number.parse::<u64>()?,
			value: latest_slot,
		};

		if storage.state_changes.last().unwrap() != &slot {
			println!("New storage slot value: {:?}", &slot.value);
			storage.state_changes.push(slot);
		}
	}

	Ok(())
}
