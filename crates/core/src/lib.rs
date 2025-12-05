use regex::Regex;

/// Newtype representing a validated email address.
#[derive(Debug, Clone)]
pub struct Email(String);

impl Email {
    pub fn parse(raw: &str) -> Result<Self, String> {
        let re = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$")
            .expect("hard-coded regex is valid");

        if !re.is_match(raw) {
            return Err(format!("Invalid email: {raw:?}"));
        }
        Ok(Email(raw.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn domain(&self) -> &str {
        self.0
            .split('@')
            .nth(1)
            .expect("validated email must contain '@'")
    }
}

/// Newtype representing a non-empty string.
#[derive(Debug, Clone)]
pub struct NonEmptyStr(String);

impl NonEmptyStr {
    pub fn parse(raw: &str) -> Result<Self, String> {
        if raw.is_empty() {
            return Err("String must be non-empty".to_string());
        }
        Ok(NonEmptyStr(raw.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Example domain function that uses the newtypes.
pub fn send_email(
    to: &Email,
    subject: &NonEmptyStr,
    body: &NonEmptyStr,
) {
    println!("Sending email (from Rust core):");
    println!("  To:      {} (domain: {})", to.as_str(), to.domain());
    println!("  Subject: {}", subject.as_str());
    println!("  Body:    {:?}", body.as_str());
}
