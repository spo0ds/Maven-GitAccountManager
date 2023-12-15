pub fn is_valid_username(username: &str) -> bool {
    !username.trim().is_empty()
}

pub fn is_valid_email(email: &str) -> bool {
    let email_regex =
        regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}
