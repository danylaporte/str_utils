use super::{Mapped, MappedChars, MappedTable};
use static_init::dynamic;
use std::char;
use unicode_normalization::UnicodeNormalization;

/// Convert a char to lowercase without accent.
pub fn lower_no_accent_char(c: char) -> impl Iterator<Item = char> {
    mapped_table()[c as usize]
}

/// Convert the Chars iterator to an iterator having all lowercase without accent.
pub fn lower_no_accent_chars<'a>(s: &'a str) -> MappedChars<'a> {
    MappedChars {
        chars: s.chars(),
        mapped: Mapped::Empty,
        table: mapped_table(),
    }
}

/// Get a table containing all char conversion to lowercase / without accent chars.
fn mapped_table() -> MappedTable {
    #[dynamic]
    static TABLE: Vec<Mapped> = {
        let mut chars = vec![Mapped::Empty; char::MAX as usize];

        chars.iter_mut().enumerate().for_each(|(index, c)| {
            *c = match char::from_u32(index as u32) {
                Some(c) => {
                    let mut iter = c
                        .to_lowercase()
                        .nfd()
                        .filter(|c| c.is_ascii() || c.is_alphanumeric());

                    match (iter.next(), iter.next(), iter.next()) {
                        (Some(a), Some(b), Some(c)) => Mapped::C3(a, b, c),
                        (Some(a), Some(b), _) => Mapped::C2(a, b),
                        (Some(a), _, _) => Mapped::C1(a),
                        _ => Mapped::Empty,
                    }
                }
                None => Mapped::Empty,
            };
        });

        chars
    };

    &TABLE
}

#[test]
fn test_accent_removal() {
    assert_eq!("aei", &lower_no_accent_chars("àéï").collect::<String>());
    assert_eq!("a.e i", &lower_no_accent_chars("à.é ï").collect::<String>());
    assert_eq!("123", &lower_no_accent_chars("123").collect::<String>());
    assert_eq!("", &lower_no_accent_chars("").collect::<String>());
}
