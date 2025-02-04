use std::borrow::Cow;

pub trait TrimInPlace {
    /// Trim a string in place.
    ///
    /// ```
    /// use str_utils::TrimInPlace;
    ///
    /// let mut s = " test ".to_string();
    ///
    /// s.trim_in_place();
    ///
    /// assert_eq!(&*s, "test");
    /// ```
    fn trim_in_place(&mut self) -> &mut Self;
}

impl TrimInPlace for Box<str> {
    fn trim_in_place(&mut self) -> &mut Self {
        let new = self.trim();

        if new.len() != self.len() {
            *self = new.into();
        }

        self
    }
}

impl TrimInPlace for Cow<'_, str> {
    fn trim_in_place(&mut self) -> &mut Self {
        let new = self.trim();

        if new.len() != self.len() {
            *self = Cow::Owned(new.to_owned());
        }

        self
    }
}

impl<T> TrimInPlace for Option<T>
where
    T: TrimInPlace,
{
    fn trim_in_place(&mut self) -> &mut Self {
        if let Some(s) = self {
            s.trim_in_place();
        }

        self
    }
}

impl TrimInPlace for String {
    fn trim_in_place(&mut self) -> &mut Self {
        let new = self.trim();

        if new.len() != self.len() {
            *self = new.into();
        }

        self
    }
}

impl<T: TrimInPlace> TrimInPlace for &mut T {
    fn trim_in_place(&mut self) -> &mut Self {
        (*self).trim_in_place();
        self
    }
}
