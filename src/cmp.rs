//! # Comparison module for strings and chars
//!
//! This module regroup comparison trait for equality and ordering.

use std::cmp::Ordering;
use std::str::Chars;
use unidecode::unidecode_char;

/// Trait for equality comparisons of string and chars.
///
/// This trait allow to do accent and case insensitive comparisons.
pub trait EqExt<Rhs = Self> {
    /// Accent insensitive
    ///
    /// # Example
    /// ```
    /// use str_utils::cmp::EqExt;
    ///
    /// // string comparison
    /// assert!("Café".eq_ai("Cafe"));
    ///
    /// // char comparison
    /// assert!('e'.eq_ai('é'));
    /// ```
    fn eq_ai(self, other: Rhs) -> bool
    where
        Self: Sized;

    /// Accent / Case insensitive
    ///
    /// # Example
    /// ```
    /// use str_utils::cmp::EqExt;
    ///
    /// // string comparison
    /// assert!("Café".eq_ai_ci("cafe"));
    ///
    /// // char comparison
    /// assert!('e'.eq_ai_ci('É'));
    /// ```
    fn eq_ai_ci(self, other: Rhs) -> bool
    where
        Self: Sized;

    /// Case insensitive
    ///
    /// # Example
    /// ```
    /// use str_utils::cmp::EqExt;
    ///
    /// // string comparison
    /// assert!("street".eq_ci("Street"));
    ///
    /// // char comparison
    /// assert!('C'.eq_ci('c'));
    ///
    /// // owned string comparison
    /// assert!("abc".to_owned().eq_ci("ABC"));
    /// ```
    fn eq_ci(self, other: Rhs) -> bool
    where
        Self: Sized;
}

impl EqExt for char {
    fn eq_ai(self, r: Self) -> bool
    where
        Self: Sized,
    {
        if self == r {
            return true;
        }

        let l = unidecode_char(self);
        let r = unidecode_char(r);
        l == r
    }

    fn eq_ai_ci(self, r: Self) -> bool
    where
        Self: Sized,
    {
        if self == r {
            return true;
        }

        let l = unidecode_char(self);
        let r = unidecode_char(r);

        if l == r {
            return true;
        }

        l.chars()
            .flat_map(char::to_lowercase)
            .eq(r.chars().flat_map(char::to_lowercase))
    }

    #[inline]
    fn eq_ci(self, r: Self) -> bool
    where
        Self: Sized,
    {
        self == r || self.to_lowercase().eq(r.to_lowercase())
    }
}

impl EqExt for &str {
    #[inline]
    fn eq_ai(self, r: Self) -> bool
    where
        Self: Sized,
    {
        self == r || eq_chars(self.chars(), r.chars(), EqExt::eq_ai)
    }

    #[inline]
    fn eq_ai_ci(self, r: Self) -> bool
    where
        Self: Sized,
    {
        self == r || eq_chars(self.chars(), r.chars(), EqExt::eq_ai_ci)
    }

    #[inline]
    fn eq_ci(self, r: Self) -> bool
    where
        Self: Sized,
    {
        self == r || eq_chars(self.chars(), r.chars(), EqExt::eq_ci)
    }
}

impl EqExt<&str> for &String {
    #[inline]
    fn eq_ai(self, r: &str) -> bool
    where
        Self: Sized,
    {
        self.as_str().eq_ai(r)
    }

    #[inline]
    fn eq_ai_ci(self, r: &str) -> bool
    where
        Self: Sized,
    {
        self.as_str().eq_ai_ci(r)
    }

    #[inline]
    fn eq_ci(self, r: &str) -> bool
    where
        Self: Sized,
    {
        self.as_str().eq_ci(r)
    }
}

fn eq_chars<F>(mut l: Chars, mut r: Chars, f: F) -> bool
where
    F: Fn(char, char) -> bool,
{
    loop {
        return match (l.next(), r.next()) {
            (Some(l), Some(r)) => {
                if f(l, r) {
                    continue;
                }
                false
            }
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
        };
    }
}

/// Trait for ordering of string and chars.
///
/// This trait allow to do accent and case insensitive ordering.
pub trait OrdExt<Rhs = Self> {
    /// Accent insensitive comparison.
    /// 
    /// # Example
    /// ```
    /// use std::cmp::Ordering;
    /// use str_utils::cmp::OrdExt;
    /// 
    /// assert_eq!("abc".cmp_ai("àbc"), Ordering::Equal);
    /// assert_eq!("é".cmp_ai("e"), Ordering::Equal);
    /// ```
    fn cmp_ai(self, rhs: Rhs) -> Ordering;

    /// Accent / case insensitive comparison.
    /// 
    /// # Example
    /// ```
    /// use std::cmp::Ordering;
    /// use str_utils::cmp::OrdExt;
    /// 
    /// assert_eq!("abc".cmp_ai_ci("Àbc"), Ordering::Equal);
    /// assert_eq!("é".cmp_ai_ci("E"), Ordering::Equal);
    /// ```
    fn cmp_ai_ci(self, rhs: Rhs) -> Ordering;

    /// Case insensitive comparison.
    /// 
    /// # Example
    /// ```
    /// use std::cmp::Ordering;
    /// use str_utils::cmp::OrdExt;
    /// 
    /// assert_eq!("abc".cmp_ci("Abc"), Ordering::Equal);
    /// assert_eq!("e".cmp_ci("E"), Ordering::Equal);
    /// ```
    fn cmp_ci(self, rhs: Rhs) -> Ordering;
}

impl OrdExt<char> for char {
    #[inline]
    fn cmp_ai(self, r: char) -> Ordering {
        unidecode_char(self).cmp(unidecode_char(r))
    }

    fn cmp_ai_ci(self, r: char) -> Ordering {
        unidecode_char(self)
            .chars()
            .flat_map(char::to_lowercase)
            .cmp(unidecode_char(r).chars().flat_map(char::to_lowercase))
    }

    #[inline]
    fn cmp_ci(self, r: char) -> Ordering {
        self.to_lowercase().cmp(r.to_lowercase())
    }
}

impl OrdExt<&str> for &str {
    #[inline]
    fn cmp_ai(self, r: &str) -> Ordering {
        ord_chars(self.chars(), r.chars(), OrdExt::cmp_ai)
    }

    #[inline]
    fn cmp_ai_ci(self, r: &str) -> Ordering {
        ord_chars(self.chars(), r.chars(), OrdExt::cmp_ai_ci)
    }

    #[inline]
    fn cmp_ci(self, r: &str) -> Ordering {
        ord_chars(self.chars(), r.chars(), OrdExt::cmp_ci)
    }
}

impl OrdExt<&str> for &String {
    #[inline]
    fn cmp_ai(self, r: &str) -> Ordering {
        self.as_str().cmp_ai(r)
    }

    #[inline]
    fn cmp_ai_ci(self, r: &str) -> Ordering {
        self.as_str().cmp_ai_ci(r)
    }

    #[inline]
    fn cmp_ci(self, r: &str) -> Ordering {
        self.as_str().cmp_ci(r)
    }
}

fn ord_chars<F>(mut l: Chars, mut r: Chars, f: F) -> Ordering
where
    F: Fn(char, char) -> Ordering,
{
    loop {
        return match (l.next(), r.next()) {
            (Some(l), Some(r)) => {
                let v = f(l, r);
                if v == Ordering::Equal {
                    continue;
                }
                v
            }
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        };
    }
}

#[test]
fn char_eq_ai_works() {
    assert!('a'.eq_ai('à'));
    assert!('ä'.eq_ai('a'));
    assert!('a'.eq_ai('a'));
    assert!(!'a'.eq_ai('b'));
    assert!(!'a'.eq_ai('A'));
}

#[test]
fn char_eq_ai_ci_works() {
    assert!('a'.eq_ai_ci('à'));
    assert!('a'.eq_ai_ci('À'));
    assert!('A'.eq_ai_ci('À'));
    assert!('à'.eq_ai_ci('à'));
    assert!('À'.eq_ai_ci('a'));
    assert!(!'a'.eq_ai_ci('b'));
}

#[test]
fn char_eq_ci_works() {
    assert!('a'.eq_ci('a'));
    assert!('A'.eq_ci('a'));
    assert!('a'.eq_ci('A'));
    assert!(!'a'.eq_ci('b'));
}

#[test]
fn str_eq_ai_works() {
    assert!("abc".eq_ai("àbc"));
    assert!(!"abc".eq_ai("abca"));
}

#[test]
fn str_eq_ai_ci_works() {
    assert!("abc".eq_ai_ci("ÀBc"));
    assert!(!"abc".eq_ai_ci("abca"));
}

#[test]
fn str_eq_ci_works() {
    assert!("abc".eq_ci("ABC"));
    assert!(!"abc".eq_ci("abca"));
}
