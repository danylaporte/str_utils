use super::MappedChars;

/// Convert a char to lowercase without accent.
pub fn lower_no_accent_char(c: char) -> std::str::Chars<'static> {
    let map = include_bytes!(concat!(env!("OUT_DIR"), "/map.bin"));

    let index = (c as usize) * 10;
    let len = unsafe { *map.get_unchecked(index) } as usize;

    let lbound = index + 1;
    let ubound = lbound + len;

    unsafe {
        let s = map.get_unchecked(lbound..ubound);
        std::str::from_utf8_unchecked(s)
    }
    .chars()
}

/// Convert the Chars iterator to an iterator having all lowercase without accent.
pub fn lower_no_accent_chars(s: &str) -> MappedChars {
    MappedChars {
        chars: s.chars(),
        mapped: "".chars(),
    }
}

#[test]
fn test_accent_removal() {
    assert_eq!("aei", &lower_no_accent_chars("àéï").collect::<String>());
    assert_eq!("a.e i", &lower_no_accent_chars("à.é ï").collect::<String>());
    assert_eq!("123", &lower_no_accent_chars("123").collect::<String>());
    assert_eq!("", &lower_no_accent_chars("").collect::<String>());
}
