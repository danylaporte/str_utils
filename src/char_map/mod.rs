mod lower;
mod lower_no_accent;

pub use lower::*;
pub use lower_no_accent::*;
use std::str::Chars;

#[derive(Clone, Copy)]
enum Mapped {
    Empty,
    C1(char),
    C2(char, char),
    C3(char, char, char),
}

impl Iterator for Mapped {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Self::Empty => None,
            Self::C1(a) => {
                *self = Self::Empty;
                Some(a)
            }
            Self::C2(a, b) => {
                *self = Self::C1(b);
                Some(a)
            }
            Self::C3(a, b, c) => {
                *self = Self::C2(b, c);
                Some(a)
            }
        }
    }
}

type MappedTable = &'static [Mapped];

/// Map chars to other chars.
#[derive(Clone)]
pub struct MappedChars<'a> {
    chars: Chars<'a>,
    mapped: Mapped,
    table: MappedTable,
}

impl<'a> Eq for MappedChars<'a> {}

impl<'a> Iterator for MappedChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.mapped.next();
        if c.is_some() {
            return c;
        }

        let c = self.chars.next()?;
        self.mapped = self.table[c as usize];
        self.mapped.next()
    }
}

impl<'a> PartialEq for MappedChars<'a> {
    fn eq(&self, other: &Self) -> bool {
        Iterator::eq(self.clone(), other.clone())
    }
}

impl<'a> PartialOrd for MappedChars<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for MappedChars<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Iterator::cmp(self.clone(), other.clone())
    }
}
