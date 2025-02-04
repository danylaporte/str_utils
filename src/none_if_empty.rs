use crate::form_str::FormStr;
use std::{borrow::Cow, ops::Deref};

pub trait NoneIfEmpty {
    type Output;

    fn none_if_empty(self) -> Option<Self::Output>;

    fn none_if_trim_empty(self) -> Option<Self::Output>;
}

impl NoneIfEmpty for Box<str> {
    type Output = Box<str>;

    fn none_if_empty(self) -> Option<Self::Output> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    fn none_if_trim_empty(self) -> Option<Self::Output> {
        if self.trim().is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a> NoneIfEmpty for Cow<'a, str> {
    type Output = Cow<'a, str>;

    fn none_if_empty(self) -> Option<Self::Output> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    fn none_if_trim_empty(self) -> Option<Self::Output> {
        if self.trim().is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl<F> NoneIfEmpty for FormStr<F> {
    type Output = FormStr<F>;

    fn none_if_empty(self) -> Option<Self::Output> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    fn none_if_trim_empty(self) -> Option<Self::Output> {
        if self.trim().is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl<T> NoneIfEmpty for Option<T>
where
    T: NoneIfEmpty,
{
    type Output = T::Output;

    fn none_if_empty(self) -> Option<Self::Output> {
        match self {
            Some(s) => s.none_if_empty(),
            _ => None,
        }
    }

    fn none_if_trim_empty(self) -> Option<Self::Output> {
        match self {
            Some(s) => s.none_if_trim_empty(),
            _ => None,
        }
    }
}

impl<'a> NoneIfEmpty for &'a str {
    type Output = &'a str;

    fn none_if_empty(self) -> Option<Self::Output> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    fn none_if_trim_empty(self) -> Option<Self::Output> {
        if self.trim().is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl NoneIfEmpty for String {
    type Output = String;

    fn none_if_empty(self) -> Option<Self::Output> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    fn none_if_trim_empty(self) -> Option<Self::Output> {
        if self.trim().is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a, T> NoneIfEmpty for &'a T
where
    T: Deref<Target = str>,
{
    type Output = &'a T;

    fn none_if_empty(self) -> Option<Self::Output> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    fn none_if_trim_empty(self) -> Option<Self::Output> {
        if self.trim().is_empty() {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a, T> NoneIfEmpty for &'a mut T
where
    T: Deref<Target = str>,
{
    type Output = &'a mut T;

    fn none_if_empty(self) -> Option<Self::Output> {
        if self.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    fn none_if_trim_empty(self) -> Option<Self::Output> {
        if self.trim().is_empty() {
            None
        } else {
            Some(self)
        }
    }
}
