//use crate::rpc::rpc::*;

pub fn format_hex (hex: &str) -> &str {
	// if `hex` is "\"0x8a165b\"" only return 0x8a165b
	// if `hex` is "0x8a165b" only return 0x8a165b
	if hex.starts_with("\"") {
		&hex[1..hex.len()-1]
	} else {
		hex
	}
}

// Serialize block to BlockResult Struct
// pub fn serialize_block(block_as_str: &str) -> Result<BlockResult, serde_json::Error> {
// 	let block: BlockResult = serde_json::from_str(block_as_str)?;
// 	Ok(block)
// }