use super::MappedChars;
use std::{iter::FusedIterator, str::Chars};

include!(concat!(env!("OUT_DIR"), "/char_map.rs"));

const EMPTY: u16 = 0;
const IDENTITY: u16 = 1;
const FIRST_ENTRY: u16 = 2;

/// Iterator returned by [lower_no_accent_char].
#[derive(Clone, Debug)]
pub struct LowerNoAccentChar {
    single: Option<char>,
    rest: Chars<'static>,
}

impl LowerNoAccentChar {
    #[inline]
    fn single(c: Option<char>) -> Self {
        Self {
            single: c,
            rest: "".chars(),
        }
    }
}

impl Iterator for LowerNoAccentChar {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<char> {
        match self.single.take() {
            Some(c) => Some(c),
            None => self.rest.next(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.single.is_some() as usize;
        let (lo, hi) = self.rest.size_hint();
        (lo + n, hi.map(|hi| hi + n))
    }
}

impl FusedIterator for LowerNoAccentChar {}

/// Convert a char to lowercase without accent.
#[inline]
pub fn lower_no_accent_char(c: char) -> LowerNoAccentChar {
    if c.is_ascii() {
        return LowerNoAccentChar::single(Some(c.to_ascii_lowercase()));
    }

    let cp = c as usize;
    let block = LEVEL1[cp >> 8] as usize;

    match BLOCKS[(block << 8) | (cp & 0xFF)] {
        EMPTY => LowerNoAccentChar::single(None),
        IDENTITY => LowerNoAccentChar::single(Some(c)),
        id => {
            let packed = ENTRIES[(id - FIRST_ENTRY) as usize];
            let offset = (packed >> 8) as usize;
            let len = (packed & 0xFF) as usize;

            LowerNoAccentChar {
                single: None,
                rest: POOL[offset..offset + len].chars(),
            }
        }
    }
}

/// Convert the Chars iterator to an iterator having all lowercase without accent.
pub fn lower_no_accent_chars(s: &str) -> MappedChars<'_> {
    MappedChars {
        chars: s.chars(),
        mapped: LowerNoAccentChar::single(None),
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
