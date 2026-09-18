use crate::char_map::{MappedChar, no_accent_char};

/// A trait to remove accent on a char.
pub trait CharExt {
    /// Remove accent on a char.
    ///
    /// # Example
    /// ```
    /// use str_utils::CharExt;
    ///
    /// assert_eq!('é'.no_accent().collect::<String>(), "e");
    /// ```
    fn no_accent(self) -> MappedChar
    where
        Self: Sized;
}

impl CharExt for char {
    #[inline]
    fn no_accent(self) -> MappedChar {
        no_accent_char(self)
    }
}

#[test]
fn no_accent_works() {
    assert_eq!('à'.no_accent().collect::<String>(), "a");
    assert_eq!('À'.no_accent().collect::<String>(), "A");
}
