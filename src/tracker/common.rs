use crate::hex_to_decimal;
use crate::tracker::types::SerializeStorage;
use crate::tracker::time::get_latest_unix_timestamp;

use regex::Regex;
use std::fs;

pub fn set_filename_and_serialize<T: SerializeStorage>(
	path: String,
	filename: String,
	storage: T,
	contract_address: String,
	middle_label: &str,
	middle_value: String,
	output_as_decimal: bool,
) -> Result<(), Box<dyn std::error::Error>> {
	let mut json: String;

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

	if output_as_decimal {
		json = output_to_dec(json);
	}

	fs::write(path, json)?;
	
	Ok(())
}

fn is_valid_eth_address(hex_str: &str) -> bool {
    hex_str.len() == 42
        && hex_str.starts_with("0x")
        && hex_str[2..].chars().all(|c| c.is_ascii_hexdigit())
}

// Find all hex numbers and convert them to decimals, ignore eth addresses
fn output_to_dec(json: String) -> String {
    // Regular expression to find hexadecimal numbers in the JSON string
    let re = Regex::new(r#"(?i)\b0x[0-9a-f]+\b"#).unwrap();

    let result = re.replace_all(&json, |caps: &regex::Captures| {
        let hex_str = caps.get(0).unwrap().as_str();

        // If it's a valid Ethereum address, don't convert and keep as is
        if is_valid_eth_address(hex_str) {
            hex_str.to_string()
        } else {
            // Convert the hexadecimal to decimal and return as a string
            hex_to_decimal(hex_str).unwrap().to_string()
        }
    });

    result.to_string()
}