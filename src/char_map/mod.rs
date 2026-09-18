mod human_cmp;
mod lower;
mod lower_no_accent;
mod no_accent;
mod table;

pub use human_cmp::human_cmp;
pub use lower::*;
pub use lower_no_accent::*;
pub use no_accent::*;
use std::str::Chars;
pub use table::MappedChar;

/// Map chars to other chars.
#[derive(Clone)]
pub struct MappedChars<'a> {
    chars: Chars<'a>,
    mapped: MappedChar,
}

impl Eq for MappedChars<'_> {}

impl Iterator for MappedChars<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.mapped.next() {
            return Some(c);
        }

        for c in self.chars.by_ref() {
            self.mapped = lower_no_accent_char(c);

            if let Some(c) = self.mapped.next() {
                return Some(c);
            }
        }

        None
    }
}

impl PartialEq for MappedChars<'_> {
    fn eq(&self, other: &Self) -> bool {
        Iterator::eq(self.clone(), other.clone())
    }
}

impl PartialOrd for MappedChars<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MappedChars<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Iterator::cmp(self.clone(), other.clone())
    }
}
