use crate::tracker::time::get_latest_unix_timestamp;
use crate::tracker::types::StateChangeList;
use crate::U256;

use std::fs;

pub fn set_filename_and_serialize(
	path: String,
	filename: String,
	storage: StateChangeList,
	contract_address: String,
	storage_slot: U256,
) -> Result<(), Box<dyn std::error::Error>> {
	let mut is_csv = false;
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
