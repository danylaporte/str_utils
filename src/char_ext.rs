use unidecode::unidecode_char;

/// A trait to remove accent on a char.
pub trait CharExt {
    /// Remove accent on a char.
    /// 
    /// # Example
    /// ```
    /// use str_utils::CharExt;
    /// 
    /// assert_eq!('é'.no_accent(), "e");
    /// ```
    fn no_accent(self) -> &'static str
    where
        Self: Sized;
}

impl CharExt for char {
    fn no_accent(self) -> &'static str {
        unidecode_char(self)
    }
}
