use crate::cmp::EqExt;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq)]
pub enum FsError {
    EmptySegment,
    EndsWithDot,
    Home,
    InvalidChar(char),
    ReservedName(&'static str),
    Root,
}

pub type Result<T> = std::result::Result<T, FsError>;

impl Debug for FsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySegment => f.write_str("empty segment"),
            Self::EndsWithDot => f.write_str("filename ends with dot"),
            Self::Home => f.write_str("path refer to ~"),
            Self::InvalidChar(c) => write!(f, "invalid filename char {c}"),
            Self::ReservedName(s) => write!(f, "reserved file name {s}"),
            Self::Root => f.write_str("path cannot be root"),
        }
    }
}

impl Display for FsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

/// Apply the validation of the [validate_sub_path] function. If it works, it format
/// the path by triming segment.
pub fn format_sub_path(s: &str) -> Result<String> {
    validate_sub_path(s)?;

    let mut out = String::new();

    for s in s.split(['/', '\\']) {
        out.push_str(s.trim());
    }

    Ok(out)
}

/// Format a filename such as "test.txt" and validate it.
///
/// Since filename must always be trimmed, the function returns the trimmed input string.
pub fn validate_filename(s: &str) -> Result<&str> {
    for c in s.chars() {
        if c.is_control()
            || (c as u32) < 31
            || c == '<'
            || c == '>'
            || c == ':'
            || c == '"'
            || c == '/'
            || c == '\\'
            || c == '|'
            || c == '?'
            || c == '*'
        {
            return Err(FsError::InvalidChar(c));
        }
    }

    // files must always be trimmed.
    let s = s.trim();

    if s.ends_with('.') {
        return Err(FsError::EndsWithDot);
    }

    for reserved in [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ] {
        if s.eq_ci(reserved) {
            return Err(FsError::ReservedName(reserved));
        }
    }

    Ok(s)
}

/// Validate a path to a file. The path must not be rooted, and must not contains chars allowing it to escape a folder.
///
/// #Example
///
/// ```
/// use str_utils::fs::validate_sub_path;
///
/// assert!(validate_sub_path("sub_dir/file.txt").is_ok());
/// assert!(validate_sub_path("/file.txt").is_err());
/// assert!(validate_sub_path("./file.txt").is_err());
/// assert!(validate_sub_path("../file.txt").is_err());
/// assert!(validate_sub_path("~/file.txt").is_err());
/// assert!(validate_sub_path("\\\\test\\file.txt").is_err());
/// assert!(validate_sub_path("c:\\file.txt").is_err());
/// ```
pub fn validate_sub_path(s: &str) -> Result<()> {
    let s = s.trim();

    if s.starts_with('/') || s.starts_with("\\") {
        return Err(FsError::Root);
    }

    if s.starts_with("~/") {
        return Err(FsError::Home);
    }

    for s in s.split(['/', '\\']) {
        let s = s.trim();

        if s.is_empty() {
            return Err(FsError::EmptySegment);
        }

        validate_filename(s)?;
    }

    Ok(())
}

#[test]
fn test_validate_sub_path() {
    assert!(validate_sub_path("test/~").is_ok());
    assert!(validate_sub_path("test~/as.txt").is_ok());
    assert!(validate_sub_path("~").is_ok());
    assert!(validate_sub_path("~/").is_err());
    assert!(validate_sub_path("~t~/as.txt").is_ok());
    assert!(validate_sub_path("/").is_err());
    assert!(validate_sub_path("txt.").is_err());
    assert!(validate_sub_path("./").is_err());
    assert!(validate_sub_path("../").is_err());
    assert!(validate_sub_path("test/..txt").is_ok());
    assert!(validate_sub_path("do_not_have_1_dots_in_path/./test.txt").is_err());
    assert!(validate_sub_path("do_not_have_1_dots_in_path/./test.txt").is_err());
    assert!(validate_sub_path("do_not_have_2_dots_in_path/../test.txt").is_err());
    assert!(validate_sub_path("do_not_end_with_dot_in_path./text.txt").is_err());
    assert!(validate_sub_path("path/do_not_end_with.").is_err());
}

//ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~:/?#[]@!$&'()*+,;=
