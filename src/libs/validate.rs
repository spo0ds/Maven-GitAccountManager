/// Checks if a username is valid.
///
/// A valid username must not be empty after trimming whitespace.

pub fn is_valid_username(username: &str) -> bool {
    !username.trim().is_empty()
}

/// Checks if an email address is valid.
///
/// A valid email address must conform to the following pattern:
/// - Start with one or more alphanumeric characters, underscores, dots, plus signs or hyphens.
/// - Followed by an "@" symbol.
/// - Followed by one or more alphanumeric characters, dots or hyphens in the domain name.
/// - End with a two-letter or more country code extension.

pub fn is_valid_email(email: &str) -> bool {
    let email_regex =
        regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}
