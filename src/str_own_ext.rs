use std::ops::Deref;

pub trait StrOwnExt {
    /// Trim a string in place.
    ///
    /// ```
    /// use str_utils::StrOwnExt;
    ///
    /// let mut s = " test ".to_string();
    ///
    /// s.trim_in_place();
    ///
    /// assert_eq!(&*s, "test");
    /// ```
    fn trim_in_place(&mut self) -> &mut Self;
}

impl<T> StrOwnExt for T
where
    T: Deref<Target = str> + for<'a> From<&'a str>,
{
    fn trim_in_place(&mut self) -> &mut Self {
        let s = self.trim();

        if s.len() != self.len() {
            *self = T::from(s);
        }

        self
    }
}
