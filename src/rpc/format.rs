//use crate::rpc::rpc::*;

pub fn format_hex(hex: &str) -> &str {
    // if `hex` is "\"0x8a165b\"" only return 0x8a165b
    // if `hex` is "0x8a165b" only return 0x8a165b
    if hex.starts_with("\"") {
        &hex[1..hex.len() - 1]
    } else {
        hex
    }
}

pub fn hex_to_decimal(hex_string: &str) -> Result<u64, std::num::ParseIntError> {
    // remove 0x prefix if it exists
    let hex_string = if hex_string.starts_with("0x") {
        &hex_string[2..]
    } else {
        hex_string
    };

    u64::from_str_radix(hex_string, 16)
}

pub fn decimal_to_hex(decimal: u64) -> String {
    format!("0x{:x}", decimal)
}

// If input doesnt have 0x in front, treat it as decimal and convert to hex
pub fn format_number_input(block: &str) -> String {
    if block.starts_with("0x") {
        block.to_string()
    } else {
        decimal_to_hex(block.parse::<u64>().unwrap())
    }
}

// Serialize block to BlockResult Struct
// pub fn serialize_block(block_as_str: &str) -> Result<BlockResult, serde_json::Error> {
// 	let block: BlockResult = serde_json::from_str(block_as_str)?;
// 	Ok(block)
// }
