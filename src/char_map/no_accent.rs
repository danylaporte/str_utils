use super::table::{MappedChar, Table};

mod map {
    include!(concat!(env!("OUT_DIR"), "/no_accent_map.rs"));
}

static TABLE: Table = Table {
    level1: &map::LEVEL1,
    blocks: &map::BLOCKS,
    entries: &map::ENTRIES,
    pool: map::POOL,
};

/// Remove accent on a char, preserving case.
#[inline]
pub fn no_accent_char(c: char) -> MappedChar {
    if c.is_ascii() {
        return MappedChar::single(Some(c));
    }

    TABLE.lookup(c)
}

/// Checks the compact table against the reference definition for every char.
#[test]
fn matches_reference_for_all_chars() {
    use unicode_normalization::UnicodeNormalization;

    let mut expected = String::new();
    let mut actual = String::new();

    for c in (0..=char::MAX as u32).filter_map(char::from_u32) {
        expected.clear();
        expected.extend(c.nfd().filter(|c| c.is_ascii() || c.is_alphanumeric()));

        actual.clear();
        actual.extend(no_accent_char(c));

        assert_eq!(actual, expected, "mismatch for U+{:04X}", c as u32);
    }
}

#[test]
fn test_accent_removal() {
    assert_eq!(
        "aEi",
        &"àÉï".chars().flat_map(no_accent_char).collect::<String>()
    );
    assert_eq!("A", &no_accent_char('À').collect::<String>());
    assert_eq!(
        "123",
        &"123".chars().flat_map(no_accent_char).collect::<String>()
    );
}
