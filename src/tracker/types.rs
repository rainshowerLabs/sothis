use serde::{Deserialize, Serialize};
use ethers::types::U256;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct StateChange {
	pub block_number: U256,
	pub value: String,
}

impl StateChange {
    pub fn serialize_csv(&self) -> String {
        format!("{},{}", self.block_number, self.value)
    }

    // pub fn deserialize_csv(csv: &str) -> Result<Self, Box<dyn std::error::Error>> {
    //     let mut split = csv.split(',');
    //     let block_number = split.next().unwrap().parse::<U256>()?;
    //     let value = split.next().unwrap().to_string();
    //     Ok(StateChange {
    //         block_number,
    //         value,
    //     })
    // }

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
    pub fn serialize_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    // pub fn deserialize_from_json(json: &str) -> Result<Self, serde_json::Error> {
    //     serde_json::from_str(json)
    // }

    // When we serialize to csv, we format it as block_number,value
    pub fn serialize_csv(&self) -> String {
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

#[allow(dead_code)]
impl CallChangeList {
    pub fn serialize_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    // pub fn deserialize_from_json(json: &str) -> Result<Self, serde_json::Error> {
    //     serde_json::from_str(json)
    // }

    // When we serialize to csv, we format it as block_number,value
    pub fn serialize_csv(&self) -> String {
        let mut csv = String::new();
        for change in &self.state_changes {
            csv.push_str(&change.serialize_csv());
            csv.push('\n');
        }
        csv
    }
}
