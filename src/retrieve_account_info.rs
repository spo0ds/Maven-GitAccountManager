#![deny(clippy::all)]
use crate::get_account_info_user::{load_accounts, Account};

pub fn retrieve_account_info(username: String, accounts_file: &str) -> Option<Account> {
    if let Ok(accounts) = load_accounts(accounts_file) {
        accounts.get(&username).cloned()
    } else {
        return None;
    }
}
