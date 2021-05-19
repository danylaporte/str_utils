mod lower_no_accent;

use std::str::Chars;

pub use lower_no_accent::*;

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
