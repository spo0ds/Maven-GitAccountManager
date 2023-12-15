use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn save_accounts(
    accounts: &HashMap<String, String>,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let json_data = serde_json::to_string_pretty(accounts)?;
    fs::write(filename, json_data)?;
    Ok(())
}

pub fn load_accounts(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    fs::read_to_string(filename)
        .map(|file_content| serde_json::from_str(&file_content).unwrap_or_default())
        .or_else(|_| Ok(HashMap::new()))
}
