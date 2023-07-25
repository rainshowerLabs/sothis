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

impl StateChangeList {
    pub fn serialize_to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    // pub fn deserialize_from_json(json: &str) -> Result<Self, serde_json::Error> {
    //     serde_json::from_str(json)
    // }

    // When we serialize to csv, we format it as block_number,value
    pub fn serialize_to_csv(&self) -> String {
        let mut csv = String::new();
        for change in &self.state_changes {
            csv.push_str(&format!("{},{}\n", change.block_number, change.value));
        }
        csv
    }
}

