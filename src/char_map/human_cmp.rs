use super::lower_no_accent_chars;
use std::{cmp::Ordering, iter::Peekable, str::CharIndices};

#[inline]
pub fn human_cmp(a: &str, b: &str) -> Ordering {
    Iter::new(a).cmp(Iter::new(b))
}

struct Iter<'a> {
    iter: Peekable<CharIndices<'a>>,
    s: &'a str,
}

impl<'a> Iter<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            iter: s.char_indices().peekable(),
            s,
        }
    }

    fn take_while(&mut self, is_num: bool) -> Option<usize> {
        while let Some((index, c)) = self.iter.peek() {
            if c.is_numeric() == is_num {
                self.iter.next();
            } else {
                return Some(*index);
            }
        }

        None
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = WordOrNumber<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some((start, c)) => {
                let is_num = c.is_numeric();

                let s = if let Some(end) = self.take_while(is_num) {
                    &self.s[start..end]
                } else {
                    &self.s[start..]
                };

                Some(if is_num {
                    WordOrNumber::Number(s)
                } else {
                    WordOrNumber::Other(s)
                })
            }
            None => None,
        }
    }
}

enum WordOrNumber<'a> {
    Number(&'a str),
    Other(&'a str),
}

impl<'a> Eq for WordOrNumber<'a> {}

impl<'a> PartialEq for WordOrNumber<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => {
                l.trim_start_matches('0') == r.trim_start_matches('0')
            }
            (Self::Number(l), Self::Other(r)) | (Self::Other(l), Self::Number(r)) => l == r,
            (Self::Other(l), Self::Other(r)) => {
                lower_no_accent_chars(l).eq(lower_no_accent_chars(r))
            }
        }
    }
}

impl<'a> PartialOrd for WordOrNumber<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for WordOrNumber<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => {
                let l = l.trim_start_matches('0');
                let r = r.trim_start_matches('0');

                l.len().cmp(&r.len()).then_with(|| l.cmp(r))
            }
            (Self::Number(l), Self::Other(r)) | (Self::Other(l), Self::Number(r)) => l.cmp(r),
            (Self::Other(l), Self::Other(r)) => {
                lower_no_accent_chars(l).cmp(lower_no_accent_chars(r))
            }
        }
    }
}

#[test]
fn test_human_cmp() {
    assert_eq!(human_cmp("", "1"), Ordering::Less);
    assert_eq!(human_cmp("1", ""), Ordering::Greater);
    assert_eq!(human_cmp("10", "1"), Ordering::Greater);
    assert_eq!(human_cmp("010", "1"), Ordering::Greater);
    assert_eq!(human_cmp("-010", "1"), Ordering::Less);
    assert_eq!(human_cmp("a", "b"), Ordering::Less);
    assert_eq!(human_cmp("a1ac1", ""), Ordering::Greater);
    assert_eq!(human_cmp("1aa1aaaaa", "1aa1aaaaa"), Ordering::Equal);

    assert_eq!(
        human_cmp("Radka Vlá?elová", "Radka Vlá?elová"),
        Ordering::Equal
    );

    assert_eq!(human_cmp("4é", "4e"), Ordering::Equal);
    assert_eq!(human_cmp("4ç", "4c"), Ordering::Equal);
    assert_eq!(human_cmp("4Ç", "4C"), Ordering::Equal);
}
