use prettytable::{row, Cell, Row, Table};
use std::collections::HashMap;

pub fn display_account_list(accounts: &HashMap<String, String>) {
    let mut table = Table::new();
    table.add_row(row!["Username", "Email"]);

    for (username, email) in accounts.iter() {
        table.add_row(Row::new(vec![Cell::new(username), Cell::new(email)]));
    }

    table.printstd();
}
