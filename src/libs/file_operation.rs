use std::collections::HashMap;
use std::error::Error;
use std::fs;

/// Saves a HashMap of usernames and emails to a JSON file.
///
/// # Arguments
///
/// * `accounts` - A reference to a HashMap where the keys are usernames and the values are emails.
/// * `filename` - The name of the file to save the accounts to.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - An `Ok` value if the accounts are saved successfully, or an `Err` containing the error if something goes wrong.

pub fn save_accounts(
    accounts: &HashMap<String, String>,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let json_data = serde_json::to_string_pretty(accounts)?;
    fs::write(filename, json_data)?;
    Ok(())
}

/// Loads a HashMap of usernames and emails from a JSON file.
///
/// # Arguments
///
/// * `filename` - The name of the file to load the accounts from.
///
/// # Returns
///
/// * `Result<HashMap<String, String>, Box<dyn Error>>` - An `Ok` value containing the loaded HashMap if successful, or an `Err` containing the error if something goes wrong.

pub fn load_accounts(filename: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    fs::read_to_string(filename)
        .map(|file_content| serde_json::from_str(&file_content).unwrap_or_default())
        .or_else(|_| Ok(HashMap::new()))
}
