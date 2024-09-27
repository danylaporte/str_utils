use std::fmt::{self, Debug, Display, Formatter};

pub enum UrlError {
    InvalidChar(char),
}

impl Debug for UrlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidChar(c) => write!(f, "invalid url char {c}"),
        }
    }
}

impl Display for UrlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

pub type Result<T> = std::result::Result<T, UrlError>;

/// Ensure all chars in the string are accepted for an url.
pub fn validate_accepted_url_chars(s: &str) -> Result<()> {
    let s = s.trim();

    const VALID_CHARS: [char; 23] = [
        '-', '.', '_', '~', ':', '/', '?', '#', '[', ']', '@', '!', '$', '&', '\'', '(', ')', '*',
        '+', ',', ';', '=', '%',
    ];

    for c in s.chars() {
        //ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~:/?#[]@!$&'()*+,;=%
        if !c.is_ascii_alphanumeric() && !VALID_CHARS.contains(&c) {
            return Err(UrlError::InvalidChar(c));
        }
    }

    Ok(())
}

#[test]
fn test_validate_accepted_url_chars() {
    assert!(
        validate_accepted_url_chars("http://est.jon/v?q=param&other=param%20#test=123").is_ok()
    );
}
