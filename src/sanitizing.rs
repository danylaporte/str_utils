use std::borrow::Cow;

pub fn is_valid_char(c: char) -> bool {
    (c as u32) >= 32 || ['\x07', '\x08', '\x0B', '\x0C', '\n', '\r', '\t'].contains(&c)
}

pub fn sanitize(s: &str) -> Cow<'_, str> {
    if s.chars().all(|c| is_valid_char(c)) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.chars().filter(|c| is_valid_char(*c)).collect())
    }
}

#[test]
fn test_is_valid_char() {
    assert!(!is_valid_char(''));
    assert!(!is_valid_char('\x00'));
    assert!(is_valid_char(' '));
    assert!(is_valid_char('2'));
    assert!(is_valid_char('A'));
    assert!(is_valid_char('\\'));
    assert!(is_valid_char('\n'));
    assert!(is_valid_char('\r'));
    assert!(is_valid_char('\t'));
    assert!(is_valid_char('a'));
    assert!(is_valid_char('à'));
    assert!(is_valid_char('é'));
}

#[test]
fn test_sanitize() {
    assert_eq!(&*sanitize("ab"), "ab");
    assert_eq!(&*sanitize(" Ab31_ "), " Ab31_ ");
}
