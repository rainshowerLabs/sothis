use serde::{Deserialize, Serialize};
use ethers::types::U256;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct StateChange {
	pub block_number: U256,
	pub value: String,
}

impl Default for StateChange {
    fn default() -> Self {
        StateChange {
            block_number: 0.into(),
            value: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StateChangeList {
    pub address: String,
    pub storage_slot: U256,
	pub state_changes: Vec<StateChange>,
}
