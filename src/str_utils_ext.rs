use super::{cmp::EqExt, CharExt};
use std::borrow::Cow;

/// Trait for searching string with accent / case insensitive comparison.
pub trait StrUtilsExt {
    /// Returns true if the given pattern matches a suffix of this string slice.
    /// Returns false if it does not.
    ///
    /// The comparison is accent insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert!("Press Café".ends_with_ai("Cafe"));
    /// ```
    fn ends_with_ai(&self, pat: &str) -> bool;

    /// Returns true if the given pattern matches a suffix of this string slice.
    /// Returns false if it does not.
    ///
    /// The comparison is accent / case insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert!("Press Café".ends_with_ai_ci("cafe"));
    /// ```
    fn ends_with_ai_ci(&self, pat: &str) -> bool;

    /// Returns true if the given pattern matches a suffix of this string slice.
    /// Returns false if it does not.
    ///
    /// The comparison is case insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert!("Press Cafe".ends_with_ci("cafe"));
    /// ```
    fn ends_with_ci(&self, pat: &str) -> bool;

    /// Returns the byte index of the first character of this string slice that matches the pattern.
    /// Returns None if the pattern doesn't match.
    ///
    /// The comparison is accent insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// let s = "Löwe 老虎 Léopard";
    ///
    /// assert_eq!(s.find_ai("L"), Some(0));
    /// assert_eq!(s.find_ai("é"), Some(4));
    /// assert_eq!(s.find_ai("Leopard"), Some(13));
    /// ```
    fn find_ai(&self, pat: &str) -> Option<usize>;

    /// Returns the byte index of the first character of this string slice that matches the pattern.
    /// Returns None if the pattern doesn't match.
    ///
    /// The comparison is accent / case insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// let s = "Löwe 老虎 Léopard";
    ///
    /// assert_eq!(s.find_ai_ci("l"), Some(0));
    /// assert_eq!(s.find_ai_ci("É"), Some(4));
    /// assert_eq!(s.find_ai_ci("leopard"), Some(13));
    /// ```
    fn find_ai_ci(&self, pat: &str) -> Option<usize>;

    /// Returns the byte index of the first character of this string slice that matches the pattern.
    /// Returns None if the pattern doesn't match.
    ///
    /// The comparison is accent / case insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// let s = "Löwe 老虎 Léopard";
    ///
    /// assert_eq!(s.find_ci("l"), Some(0));
    /// assert_eq!(s.find_ci("E"), Some(4));
    /// assert_eq!(s.find_ci("léopard"), Some(13));
    /// ```
    fn find_ci(&self, pat: &str) -> Option<usize>;

    /// Returns true if the given pattern matches a prefix of this string slice.
    /// Returns false if it does not.
    ///
    /// The comparison is accent insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert!("Café Arabica".starts_with_ai("Cafe"));
    /// ```
    fn starts_with_ai(&self, pat: &str) -> bool;

    /// Returns true if the given pattern matches a prefix of this string slice.
    /// Returns false if it does not.
    ///
    /// The comparison is accent / case insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert!("Café Arabica".starts_with_ai_ci("cafe"));
    /// ```
    fn starts_with_ai_ci(&self, pat: &str) -> bool;

    /// Returns true if the given pattern matches a prefix of this string slice.
    /// Returns false if it does not.
    ///
    /// The comparison is case insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert!("Cafe Arabica".starts_with_ci("cafe"));
    /// ```
    fn starts_with_ci(&self, pat: &str) -> bool;

    /// Transform into a no accent String.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert_eq!("Café Arabica".no_accent(), "Cafe Arabica");
    /// ```
    fn no_accent(&self) -> String;

    /// Transform into a no accent lowercase String.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert_eq!("Café Arabica".no_accent_lowercase(), "cafe arabica");
    /// ```
    fn no_accent_lowercase(&self) -> String;

    /// Transform into a no accent uppercase String.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert_eq!("Café Arabica".no_accent_uppercase(), "CAFE ARABICA");
    /// ```
    fn no_accent_uppercase(&self) -> String;

    /// Returns true if the given pattern matches a sub-slice of this string slice.
    /// Returns false if it does not.
    ///
    /// The comparison is accent insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert!("Café Arabica".contains_ai("Cafe"));
    /// ```
    #[inline]
    fn contains_ai(&self, pat: &str) -> bool {
        self.find_ai(pat).is_some()
    }

    /// Returns true if the given pattern matches a sub-slice of this string slice.
    /// Returns false if it does not.
    ///
    /// The comparison is accent / case insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert!("Café Arabica".contains_ai_ci("cafe"));
    /// ```
    #[inline]
    fn contains_ai_ci(&self, pat: &str) -> bool {
        self.find_ai_ci(pat).is_some()
    }

    /// Returns true if the given pattern matches a sub-slice of this string slice.
    /// Returns false if it does not.
    ///
    /// The comparison is case insensitive.
    ///
    /// # Example
    /// ```
    /// use str_utils::StrUtilsExt;
    ///
    /// assert!("Café Arabica".contains_ci("café"));
    /// ```
    #[inline]
    fn contains_ci(&self, pat: &str) -> bool {
        self.find_ci(pat).is_some()
    }

    /// truncate a string based on chars count instead of bytes.
    fn truncate_chars(&self, max_chars: usize) -> Cow<str>;
}

impl StrUtilsExt for str {
    #[inline]
    fn ends_with_ai(&self, pat: &str) -> bool {
        ends_with(self, pat, EqExt::eq_ai)
    }

    #[inline]
    fn ends_with_ai_ci(&self, pat: &str) -> bool {
        ends_with(self, pat, EqExt::eq_ai_ci)
    }

    #[inline]
    fn ends_with_ci(&self, pat: &str) -> bool {
        ends_with(self, pat, EqExt::eq_ci)
    }

    #[inline]
    fn find_ai(&self, pat: &str) -> Option<usize> {
        find_str(self, pat, EqExt::eq_ai)
    }

    #[inline]
    fn find_ai_ci(&self, pat: &str) -> Option<usize> {
        find_str(self, pat, EqExt::eq_ai_ci)
    }

    #[inline]
    fn find_ci(&self, pat: &str) -> Option<usize> {
        find_str(self, pat, EqExt::eq_ci)
    }

    #[inline]
    fn starts_with_ai(&self, pat: &str) -> bool {
        starts_with(self, pat, EqExt::eq_ai)
    }

    #[inline]
    fn starts_with_ai_ci(&self, pat: &str) -> bool {
        starts_with(self, pat, EqExt::eq_ai_ci)
    }

    #[inline]
    fn starts_with_ci(&self, pat: &str) -> bool {
        starts_with(self, pat, EqExt::eq_ci)
    }

    fn no_accent(&self) -> String {
        let mut s = String::with_capacity(self.len());

        for c in self.chars() {
            s.push_str(c.no_accent());
        }

        s
    }

    fn no_accent_lowercase(&self) -> String {
        let mut s = String::with_capacity(self.len());

        for c in self
            .chars()
            .flat_map(|c| c.no_accent().chars())
            .flat_map(char::to_lowercase)
        {
            s.push(c);
        }

        s
    }

    fn no_accent_uppercase(&self) -> String {
        let mut s = String::with_capacity(self.len());

        for c in self
            .chars()
            .flat_map(|c| c.no_accent().chars())
            .flat_map(char::to_uppercase)
        {
            s.push(c);
        }

        s
    }

    fn truncate_chars(&self, max_chars: usize) -> Cow<str> {
        Cow::Borrowed(match self.char_indices().nth(max_chars) {
            None => self,
            Some((idx, _)) => &self[..idx],
        })
    }
}

impl StrUtilsExt for String {
    #[inline]
    fn ends_with_ai(&self, pat: &str) -> bool {
        ends_with(self, pat, EqExt::eq_ai)
    }

    #[inline]
    fn ends_with_ai_ci(&self, pat: &str) -> bool {
        ends_with(self, pat, EqExt::eq_ai_ci)
    }

    #[inline]
    fn ends_with_ci(&self, pat: &str) -> bool {
        ends_with(self, pat, EqExt::eq_ci)
    }

    #[inline]
    fn find_ai(&self, pat: &str) -> Option<usize> {
        find_str(self, pat, EqExt::eq_ai)
    }

    #[inline]
    fn find_ai_ci(&self, pat: &str) -> Option<usize> {
        find_str(self, pat, EqExt::eq_ai_ci)
    }

    #[inline]
    fn find_ci(&self, pat: &str) -> Option<usize> {
        find_str(self, pat, EqExt::eq_ci)
    }

    #[inline]
    fn starts_with_ai(&self, pat: &str) -> bool {
        starts_with(self, pat, EqExt::eq_ai)
    }

    #[inline]
    fn starts_with_ai_ci(&self, pat: &str) -> bool {
        starts_with(self, pat, EqExt::eq_ai_ci)
    }

    #[inline]
    fn starts_with_ci(&self, pat: &str) -> bool {
        starts_with(self, pat, EqExt::eq_ci)
    }

    #[inline]
    fn no_accent(&self) -> String {
        self.as_str().no_accent()
    }

    #[inline]
    fn no_accent_lowercase(&self) -> String {
        self.as_str().no_accent_lowercase()
    }

    #[inline]
    fn no_accent_uppercase(&self) -> String {
        self.as_str().no_accent_uppercase()
    }

    fn truncate_chars(&self, max_chars: usize) -> Cow<str> {
        self.as_str().truncate_chars(max_chars)
    }
}

fn find_str<F>(src: &str, pat: &str, f: F) -> Option<usize>
where
    F: Fn(char, char) -> bool,
{
    for (index, _) in src.char_indices() {
        return match search(src[index..].chars(), pat.chars(), &f) {
            SearchResult::Found => Some(index),
            SearchResult::NotFoundContinue => continue,
            SearchResult::NotFoundFinal => None,
        };
    }

    None
}

#[derive(Eq, PartialEq)]
enum SearchResult {
    /// Found
    Found,
    /// Not found and cannot continue searching
    NotFoundFinal,
    /// Not found and can continue searching
    NotFoundContinue,
}

#[inline]
fn ends_with<F>(src: &str, pat: &str, f: F) -> bool
where
    F: Fn(char, char) -> bool,
{
    search(src.chars().rev(), pat.chars().rev(), f) == SearchResult::Found
}

#[inline]
fn starts_with<F>(src: &str, pat: &str, f: F) -> bool
where
    F: Fn(char, char) -> bool,
{
    search(src.chars(), pat.chars(), f) == SearchResult::Found
}

fn search<F, S, P>(mut src: S, mut pat: P, f: F) -> SearchResult
where
    F: Fn(char, char) -> bool,
    S: Iterator<Item = char>,
    P: Iterator<Item = char>,
{
    loop {
        return match (src.next(), pat.next()) {
            (Some(s), Some(p)) => {
                if f(s, p) {
                    continue;
                }
                SearchResult::NotFoundContinue
            }
            (None, Some(_)) => SearchResult::NotFoundFinal,
            (_, None) => SearchResult::Found,
        };
    }
}

#[test]
fn ends_with_ai_works() {
    assert!("Café".ends_with_ai("fe"));
    assert!("Cafe".ends_with_ai("fé"));
    assert!(!"Café".ends_with_ai("FE"));

    assert!("Café".to_owned().ends_with_ai("fe"));
}

#[test]
fn ends_with_ai_ci_works() {
    assert!("Café".ends_with_ai_ci("FE"));
    assert!("Cafe".ends_with_ai_ci("FÉ"));
    assert!("CafÉ".ends_with_ai_ci("fe"));

    assert!("Café".to_owned().ends_with_ai_ci("FE"));
}

#[test]
fn ends_with_ci_works() {
    assert!("Café".ends_with_ci("FÉ"));
    assert!(!"Café".ends_with_ci("FE"));
    assert!("Café".to_owned().ends_with_ci("FÉ"));
}

#[test]
fn starts_with_ai_works() {
    assert!("Café".starts_with_ai("Cafe"));
    assert!("Cafe".starts_with_ai("Café"));
    assert!(!"Café".starts_with_ai("CaFE"));
    assert!("Café".to_owned().starts_with_ai("Cafe"));
}

#[test]
fn starts_with_ai_ci_works() {
    assert!("Café Arabica".starts_with_ai_ci("CaFE"));
    assert!("Cafe Arabica".starts_with_ai_ci("CAFÉ"));
    assert!("CafÉ Arabica".starts_with_ai_ci("Cafe"));

    assert!("Café Arabica".to_owned().starts_with_ai_ci("CAFE"));
}

#[test]
fn starts_with_ci_works() {
    assert!("Café Arabica".starts_with_ci("CAFÉ"));
    assert!(!"Café Arabica".starts_with_ci("CAFE"));
    assert!("Café Arabica".to_owned().starts_with_ci("caFÉ"));
}
