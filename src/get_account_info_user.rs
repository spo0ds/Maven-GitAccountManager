#![deny(clippy::all)]
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::collections::HashMap;
use std::fs::File;

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Account {
    pub email: String,
    pub access_token: String,
}

pub fn get_account_info() {
    let mut accounts: HashMap<String, Account> = HashMap::new();
    let accounts_file = "accounts.json";

    // Load existing accounts (if any)
    if let Ok(loaded_accounts) = load_accounts(accounts_file) {
        accounts = loaded_accounts;
    }

    loop {
        let action = get_input("Enter the action (add, list, exit)\n");

        match action.as_str() {
            "add" => {
                let mut username = get_input("Enter username");
                while username.trim().is_empty() {
                    println!("Username cannot be empty.");
                    username = get_input("Enter username");
                }
                let mut email = get_input("Enter email\n");

                while !is_valid_email(&email) {
                    println!("Invalid email address.");
                    email = get_input("Enter email");
                }

                let mut access_token = get_input("Enter access token\n");

                while access_token.trim().is_empty() {
                    println!("Access token cannot be empty.");
                    access_token = get_input("Enter access token");
                }

                let account = Account {
                    email,
                    access_token,
                };

                accounts.insert(username.clone(), account);
                println!("Account {} added successfully\n", username);
                save_accounts(&accounts, accounts_file).expect("Failed to save accounts");
            }
            "list" => {
                if accounts.is_empty() {
                    println!("No accounts found!");
                } else {
                    for (username, account_info) in accounts.iter() {
                        println!("Username: {}", username);
                        println!("Email: {}", account_info.email);
                        println!("Access Token: {}", account_info.access_token);
                        println!("---");
                    }
                }
            }
            "exit" => {
                print!("Exiting...");
                save_accounts(&accounts, accounts_file).expect("Failed to save accounts");
                break;
            }
            _ => println!("Invalid action! Please try again."),
        }
    }
}

pub fn get_input(prompt: &str) -> String {
    println!("{}:", prompt);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub fn save_accounts(
    accounts: &HashMap<String, Account>,
    filename: &str,
) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    to_writer(&mut file, accounts)?;
    Ok(())
}

pub fn load_accounts(filename: &str) -> Result<HashMap<String, Account>, std::io::Error> {
    let mut file = File::open(filename)?;
    let accounts = from_reader(&mut file)?;
    Ok(accounts)
}

fn is_valid_email(email: &str) -> bool {
    // Regular expression for a simple email validation
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}
