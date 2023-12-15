use crate::libs::{display_accounts, file_operation, validate};
use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

/// Manages user Github accounts through CLI commands.

pub fn manager() {
    let mut accounts = file_operation::load_accounts("accounts.json").unwrap_or_default();

    let matches = App::new("manager")
        .version("1.0")
        .author("Spo0ds")
        .about("Manage user Github accounts")
        .subcommand(
            SubCommand::with_name("set")
                .about("Add a new account")
                .arg(
                    Arg::with_name("username")
                        .help("Sets the username for the new account")
                        .required(true),
                )
                .arg(
                    Arg::with_name("email")
                        .help("Sets the email for the new account")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Displays all accounts but are not necessarily added to config"),
        )
        .subcommand(
            SubCommand::with_name("switch")
                .about("Switch to a different account and sets git config to that account")
                .arg(
                    Arg::with_name("username")
                        .help("Switch to the account with the given username")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("reset")
                .about("Reset Git configuration for an account or all accounts")
                .arg(
                    Arg::with_name("username")
                        .help("Reset Git configuration for the specified account")
                        .required(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Set Git configuration for a specific account")
                .arg(
                    Arg::with_name("username")
                        .help("Specify the username to set Git configuration")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("show").about("Show current Git configuration"))
        .get_matches();

    match matches.subcommand() {
        ("set", Some(add_matches)) => {
            let username = add_matches
                .value_of("username")
                .unwrap_or_default()
                .to_string();
            let email = add_matches
                .value_of("email")
                .unwrap_or_default()
                .to_string();

            handle_set_action(&mut accounts, username, email);
        }
        ("list", Some(_)) => handle_list_action(&accounts),
        ("switch", Some(switch_matches)) => {
            let username = switch_matches
                .value_of("username")
                .unwrap_or_default()
                .to_string();
            handle_switch_action(&accounts, username);
        }
        ("reset", Some(reset_matches)) => {
            if let Some(username) = reset_matches.value_of("username") {
                handle_reset_action(&mut accounts, Some(username));
            } else {
                handle_reset_action(&mut accounts, None);
            }
        }
        ("show", Some(_)) => {
            handle_show_action();
        }
        ("config", Some(config_matches)) => {
            let username = config_matches
                .value_of("username")
                .unwrap_or_default()
                .to_string();
            handle_config_action(&accounts, username);
        }
        _ => eprintln!("Invalid action! Please try again."),
    }
}

// Helper functions for specific actions:

fn handle_show_action() {
    let git_config_output = Command::new("git")
        .args(&["config", "--list"])
        .output()
        .expect("Failed to execute git config");

    if git_config_output.status.success() {
        let output_str = String::from_utf8_lossy(&git_config_output.stdout);
        println!("Current Git Configuration:\n{}", output_str);
    } else {
        println!("Failed to retrieve Git configuration. Please make sure Git is installed.");
    }
}

fn handle_config_action(accounts: &HashMap<String, String>, username: String) {
    if let Some(email) = accounts.get(&username) {
        if set_git_config(&username, &email) {
            println!("Git configuration set for account: {}", username);
        } else {
            eprintln!("Failed to set Git configuration for account: {}", username);
        }
    } else {
        eprintln!("Account not found: {}", username);
    }
}

fn handle_set_action(accounts: &mut HashMap<String, String>, username: String, email: String) {
    if validate::is_valid_username(&username) && validate::is_valid_email(&email) {
        accounts.insert(username.clone(), email);
        if let Err(err) = file_operation::save_accounts(&accounts, "accounts.json") {
            eprintln!("Failed to save accounts: {}", err);
        } else {
            println!("Account {} added successfully", username);
        }
    } else {
        eprintln!("Invalid username or email. Please try again.");
    }
}

fn handle_list_action(accounts: &HashMap<String, String>) {
    if accounts.is_empty() {
        println!("No accounts found!");
    } else {
        display_accounts::display_account_list(&accounts);
    }
}

fn handle_switch_action(accounts: &HashMap<String, String>, username: String) {
    if let Some(email) = accounts.get(&username) {
        if set_git_config(&username, &email) {
            println!("Switched to account: {}", username);
        } else {
            eprintln!("Failed to switch to account: {}", username);
        }
    } else {
        eprintln!("Account not found: {}", username);
    }
}

fn handle_reset_action(accounts: &mut HashMap<String, String>, username: Option<&str>) {
    if let Some(username) = username {
        // Check if the specified username is present in the accounts
        if let Some(_) = accounts.get(username) {
            // Reset Git configuration for the specified account
            accounts.remove(username);
            set_git_config("", ""); // Reset Git configuration

            // Set Git configuration for the next account (if any)
            if let Some((next_username, next_email)) = accounts.iter().next() {
                set_git_config(next_username, next_email);
                println!(
                    "Git configuration reset for account: {}, and set to: {}",
                    username, next_username
                );
            } else {
                file_operation::save_accounts(&accounts, "accounts.json")
                    .expect("Failed to save accounts");
                println!("Git configuration reset for the last account: {}", username);
            }
        } else {
            println!("Error: Account not found or not configured: {}", username);
        }
    } else {
        // Reset Git configuration for all accounts and remove the accounts.json file
        accounts.clear();
        set_git_config("", ""); // Reset Git configuration
        fs::remove_file("accounts.json").ok(); // Ignore if the file doesn't exist
        println!("Git configuration reset for all accounts, and accounts.json file removed");
    }
}

fn set_git_config(username: &str, email: &str) -> bool {
    fn set_config(config_key: &str, config_value: &str) -> bool {
        let result = Command::new("git")
            .args(&["config", "--global", config_key, config_value])
            .output();

        if let Err(err) = result {
            eprintln!("Failed to set git {}: {}", config_key, err);
            false
        } else {
            true
        }
    }

    set_config("user.name", username) && set_config("user.email", email)
}
