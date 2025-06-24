// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    //get the first 8 characters of the hex string
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }
    if !raw_tx_hex.is_ascii() || raw_tx_hex.chars().any(|c| !c.is_digit(16)) {
        return Err("Hex decode error".to_string());
    }
    let version_hex = &raw_tx_hex[0..8];

    let mut reversed = String::new();
    for i in (0..8).step_by(2).rev() {
        reversed.push_str(&version_hex[i..i + 2]);
    }

    let result = u32::from_str_radix(&reversed, 16)
        .map_err(|_| "Hex decode error".to_string())
        .unwrap();
    println!("Transaction version: {}", result);
    Ok(result)
}
