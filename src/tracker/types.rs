use ethers::types::U256;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct StateChange {
    pub block_number: U256,
    pub value: String,
}

impl StateChange {
    pub fn serialize_csv(&self) -> String {
        format!("{},{}", self.block_number, self.value)
    }
}

impl Default for StateChange {
    fn default() -> Self {
        StateChange {
            block_number: 0.into(),
            value: Default::default(),
        }
    }
}

pub trait SerializeStorage {
    fn serialize_json(&self) -> Result<String, serde_json::Error>;
    fn serialize_csv(&self) -> String;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StateChangeList {
    pub address: String,
    pub storage_slot: U256,
    pub state_changes: Vec<StateChange>,
}

impl SerializeStorage for StateChangeList {
    fn serialize_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    fn serialize_csv(&self) -> String {
        let mut csv = String::new();
        for change in &self.state_changes {
            csv.push_str(&change.serialize_csv());
            csv.push('\n');
        }
        csv
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CallChangeList {
    pub address: String,
    pub calldata: String,
    pub state_changes: Vec<StateChange>,
}

impl SerializeStorage for CallChangeList {
    fn serialize_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    fn serialize_csv(&self) -> String {
        let mut csv = String::new();
        for change in &self.state_changes {
            csv.push_str(&change.serialize_csv());
            csv.push('\n');
        }
        csv
    }
}
