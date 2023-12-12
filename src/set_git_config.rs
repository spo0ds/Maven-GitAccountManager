use crate::retrieve_account_info::retrieve_account_info;
use std::process::Command;

pub fn set_git_config(username: String, accounts_file: &str) -> Result<(), std::io::Error> {
    if let Some(account) = retrieve_account_info(username.clone(), accounts_file) {
        let email = &account.email;
        let mut command = Command::new("git");
        command
            .arg("config")
            .arg("--global")
            .arg("user.name")
            .arg(username.clone());
        command.output().expect("Failed to set username");

        command = Command::new("git");
        command
            .arg("config")
            .arg("--global")
            .arg("user.email")
            .arg(email);
        command.output().expect("Failed to set email");

        println!(
            "Git config successfully updated for username: {}",
            username.clone()
        );
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No account found for username",
        ))
    }
}
