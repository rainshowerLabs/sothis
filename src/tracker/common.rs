use crate::tracker::types::SerializeStorage;
use crate::tracker::time::get_latest_unix_timestamp;

use std::fs;

pub fn set_filename_and_serialize<T: SerializeStorage>(
	path: String,
	filename: String,
	storage: T,
	contract_address: String,
	middle_label: &str,
	middle_value: String,
) -> Result<(), Box<dyn std::error::Error>> {
	let json: String;

	// Set the filename to `address{contract_address}-{middle_label}-{storage_slot}-timestamp-{unix_timestamp} if its the default one
	// We also check if we should serialize it as csv
	let filename = match filename.as_str() {
		"" => {
			let timestamp = get_latest_unix_timestamp();
			println!("No filename specified, using default and formatting as JSON");
			json = storage.serialize_json()?;
			format!("address-{}-{}-{}-timestamp-{}.json", contract_address, middle_label, middle_value, timestamp)
		},
		filename if filename.contains(".csv") => {
			println!("Formatting as CSV");
			json = storage.serialize_csv();
			filename.to_string()
		},
		_ => {
			json = storage.serialize_json()?;
			filename
		},
	};

	let path = format!("{}/{}", path, filename);
	println!("\nWriting to file: {}", path);
	fs::write(path, json)?;
	
	Ok(())
}
