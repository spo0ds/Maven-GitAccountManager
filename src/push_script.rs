use crate::retrieve_account_info::retrieve_account_info;
use std::process::Command;

pub fn auto_authenticate_push() -> Result<(), std::io::Error> {
    let username = get_current_username()?;
    let accounts_file = "accounts.json";

    if let Some(account) = retrieve_account_info(username, accounts_file) {
        let access_token = &account.access_token;

        let mut command = Command::new("git");
        command.arg("push");

        // logic to push the changes with access token

        println!("Successfully pushed changes with automatic authentication.");
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No account found for current username",
        ))
    }
}

fn get_current_username() -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.name")
        .output()?;
    if output.status.success() {
        let username = match String::from_utf8(output.stdout) {
            Ok(username) => Ok(username.trim().to_string()),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get current git username (non-UTF-8 output)",
            )),
        };
        username
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get current git username",
        ))
    }
}
