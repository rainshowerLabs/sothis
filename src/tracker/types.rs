use serde::{Deserialize, Serialize};
use ethers::types::U256;

// 
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct StateChange {
	pub block_number: u64,
	pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StateChangeList {
	pub storage_slot: U256,
	pub state_changes: Vec<StateChange>,
}
