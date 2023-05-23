pub fn format_hex (hex: &str) -> &str {
	// if `hex` is "\"0x8a165b\"" only return 0x8a165b
	// if `hex` is "0x8a165b" only return 0x8a165b
	if hex.starts_with("\"") {
		&hex[1..hex.len()-1]
	} else {
		hex
	}
}