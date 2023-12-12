#![deny(clippy::all)]
mod get_account_info;
mod push_script;
mod retrieve_account_info;
mod set_git_config;

fn main() {
    get_account_info::get_account_info();
    if let Err(e) = set_git_config::set_git_config("spo0ds".to_string(), "accounts.json") {
        eprintln!("Error setting git config: {}", e);
        return;
    }
    if let Err(e) = push_script::auto_authenticate_push() {
        eprintln!("Error pushing script: {}", e);
        return;
    }
}
