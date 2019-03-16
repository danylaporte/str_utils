//! # Comparison module for strings and chars
//! 
//! # Example
//! 
//! ```
//! use str_utils::cmp::EqExt;
//! 
//! // perform an accent insensitive string comparison.
//! assert!("abcé".eq_ai("abcè"));
//! 
//! // perform an accent / case insensitive string comparison.
//! assert!("Abc".eq_ai_ci("àBc"));
//! ```
use unidecode::unidecode_char;

pub trait EqExt<Rhs = Self> {
    /// Accent insensitive
    fn eq_ai(self, other: Rhs) -> bool
    where
        Self: Sized;

    /// Accent / Case insensitive
    fn eq_ai_ci(self, other: Rhs) -> bool
    where
        Self: Sized;

    /// Case insensitive
    fn eq_ci(self, other: Rhs) -> bool
    where
        Self: Sized;
}

impl EqExt for char {
    fn eq_ai(self, b: Self) -> bool
    where
        Self: Sized,
    {
        if self == b {
            return true;
        }

        let a = unidecode_char(self);
        let b = unidecode_char(b);
        a == b
    }

    fn eq_ai_ci(self, b: Self) -> bool
    where
        Self: Sized,
    {
        if self == b {
            return true;
        }

        let a = unidecode_char(self);
        let b = unidecode_char(b);

        if a == b {
            return true;
        }

        a.chars()
            .flat_map(|a| a.to_lowercase())
            .eq(b.chars().flat_map(|b| b.to_lowercase()))
    }

    fn eq_ci(self, b: Self) -> bool
    where
        Self: Sized,
    {
        self == b || self.to_lowercase().eq(b.to_lowercase())
    }
}

impl EqExt for &str {
    fn eq_ai(self, b: Self) -> bool
    where
        Self: Sized,
    {
        if self == b {
            return true;
        }

        let mut a = self.chars();
        let mut b = b.chars();

        loop {
            match (a.next(), b.next()) {
                (Some(a), Some(b)) => {
                    if !a.eq_ai(b) {
                        return false;
                    }
                }
                (Some(_), None) | (None, Some(_)) => {
                    return false;
                }
                (None, None) => {
                    return true;
                }
            }
        }
    }

    fn eq_ai_ci(self, b: Self) -> bool
    where
        Self: Sized,
    {
        if self == b {
            return true;
        }

        let mut a = self.chars();
        let mut b = b.chars();

        loop {
            match (a.next(), b.next()) {
                (Some(a), Some(b)) => {
                    if !a.eq_ai_ci(b) {
                        return false;
                    }
                }
                (Some(_), None) | (None, Some(_)) => {
                    return false;
                }
                (None, None) => {
                    return true;
                }
            }
        }
    }

    fn eq_ci(self, b: Self) -> bool
    where
        Self: Sized,
    {
        if self == b {
            return true;
        }

        let mut a = self.chars();
        let mut b = b.chars();

        loop {
            match (a.next(), b.next()) {
                (Some(a), Some(b)) => {
                    if !a.eq_ci(b) {
                        return false;
                    }
                }
                (Some(_), None) | (None, Some(_)) => {
                    return false;
                }
                (None, None) => {
                    return true;
                }
            }
        }
    }
}

impl EqExt<&str> for &String {
    fn eq_ai(self, b: &str) -> bool
    where
        Self: Sized,
    {
        if self == b {
            return true;
        }

        let mut a = self.chars();
        let mut b = b.chars();

        loop {
            match (a.next(), b.next()) {
                (Some(a), Some(b)) => {
                    if !a.eq_ai(b) {
                        return false;
                    }
                }
                (Some(_), None) | (None, Some(_)) => {
                    return false;
                }
                (None, None) => {
                    return true;
                }
            }
        }
    }

    fn eq_ai_ci(self, b: &str) -> bool
    where
        Self: Sized,
    {
        if self == b {
            return true;
        }

        let mut a = self.chars();
        let mut b = b.chars();

        loop {
            match (a.next(), b.next()) {
                (Some(a), Some(b)) => {
                    if !a.eq_ai_ci(b) {
                        return false;
                    }
                }
                (Some(_), None) | (None, Some(_)) => {
                    return false;
                }
                (None, None) => {
                    return true;
                }
            }
        }
    }

    fn eq_ci(self, b: &str) -> bool
    where
        Self: Sized,
    {
        if self == b {
            return true;
        }

        let mut a = self.chars();
        let mut b = b.chars();

        loop {
            match (a.next(), b.next()) {
                (Some(a), Some(b)) => {
                    if !a.eq_ci(b) {
                        return false;
                    }
                }
                (Some(_), None) | (None, Some(_)) => {
                    return false;
                }
                (None, None) => {
                    return true;
                }
            }
        }
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
