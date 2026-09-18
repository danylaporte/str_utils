use super::MappedChars;
use super::table::{MappedChar, Table};

mod map {
    include!(concat!(env!("OUT_DIR"), "/lower_no_accent_map.rs"));
}

static TABLE: Table = Table {
    level1: &map::LEVEL1,
    blocks: &map::BLOCKS,
    entries: &map::ENTRIES,
    pool: map::POOL,
};

/// Convert a char to lowercase without accent.
#[inline]
pub fn lower_no_accent_char(c: char) -> MappedChar {
    if c.is_ascii() {
        return MappedChar::single(Some(c.to_ascii_lowercase()));
    }

    TABLE.lookup(c)
}

/// Convert the Chars iterator to an iterator having all lowercase without accent.
pub fn lower_no_accent_chars(s: &str) -> MappedChars<'_> {
    MappedChars {
        chars: s.chars(),
        mapped: MappedChar::single(None),
    }
}

/// Checks the compact table against the reference definition for every char.
#[test]
fn matches_reference_for_all_chars() {
    use unicode_normalization::UnicodeNormalization;

    let mut expected = String::new();
    let mut actual = String::new();

    for c in (0..=char::MAX as u32).filter_map(char::from_u32) {
        expected.clear();
        expected.extend(
            c.to_lowercase()
                .nfd()
                .filter(|c| c.is_ascii() || c.is_alphanumeric()),
        );

        actual.clear();
        actual.extend(lower_no_accent_char(c));

        assert_eq!(actual, expected, "mismatch for U+{:04X}", c as u32);
    }
}

#[test]
fn test_accent_removal() {
    assert_eq!("aei", &lower_no_accent_chars("àéï").collect::<String>());
    assert_eq!("a.e i", &lower_no_accent_chars("à.é ï").collect::<String>());
    assert_eq!("123", &lower_no_accent_chars("123").collect::<String>());
    assert_eq!("", &lower_no_accent_chars("").collect::<String>());
}
