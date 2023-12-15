use prettytable::{row, Cell, Row, Table};
use std::collections::HashMap;

/// Prints a table of usernames and emails from a HashMap.
///
/// # Arguments
///
/// * `accounts` - A reference to a HashMap where the keys are usernames and the values are emails.

pub fn display_account_list(accounts: &HashMap<String, String>) {
    let mut table = Table::new();
    table.add_row(row!["Username", "Email"]);

    for (username, email) in accounts.iter() {
        table.add_row(Row::new(vec![Cell::new(username), Cell::new(email)]));
    }

    table.printstd();
}
