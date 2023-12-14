use crate::retrieve_account_info::retrieve_account_info;
use std::process::Command;

pub fn auto_authenticate_push() -> Result<(), std::io::Error> {
    let username = get_current_username()?;
    let accounts_file = "accounts.json";

    if let Some(account) = retrieve_account_info(username, accounts_file) {
        let access_token = &account.access_token;

        // Iterate over all files in the current directory
        for entry in std::fs::read_dir(".")? {
            if let Ok(entry) = entry {
                let path_ref = &entry.path();
                let filename = path_ref.file_name().unwrap().to_str().unwrap();

                // Check if file exists
                if let Ok(metadata) = entry.path().metadata() {
                    let is_new_file = !metadata.is_file();

                    // Add file to staging area
                    let mut add_command = Command::new("git");
                    add_command.arg("add");
                    add_command.arg(filename);
                    add_command.output()?;

                    // Commit file with descriptive message
                    let message = if is_new_file {
                        format!("script {} added to the project", filename)
                    } else {
                        format!("script {} modified", filename)
                    };

                    let mut commit_command = Command::new("git");
                    commit_command.arg("commit");
                    commit_command.arg("-m");
                    commit_command.arg(&message);
                    commit_command.output()?;
                } else {
                    println!("Skipping non-existent file: {}", filename);
                }
            }
        }

        // Push changes to GitHub
        let mut push_command = Command::new("git");
        push_command.arg("push");
        push_command.arg(format!("--token={}", access_token));
        push_command.output()?;

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
