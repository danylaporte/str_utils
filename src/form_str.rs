use crate::fs::{validate_filename, FsError};
use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
};

pub type Result<T> = std::result::Result<T, FormatErr>;

/// A `Box<str>` that ensure a format.
///
/// The generic parameter is a Format to enforce.
#[derive(Clone)]
pub struct FormStr<F>(Box<str>, F);

impl<F> FormStr<F> {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<F: Format> FormStr<F> {
    pub fn new(s: &str) -> Result<Self>
    where
        F: Default,
    {
        Self::with_format(F::default(), s)
    }

    pub fn new_opt(s: &str) -> Option<Self>
    where
        F: Default,
    {
        Self::new(s).ok()
    }

    pub fn with_format(format: F, s: &str) -> Result<Self> {
        Ok(Self(
            format
                .format(Cow::Borrowed(s))?
                .into_owned()
                .into_boxed_str(),
            format,
        ))
    }
}

impl<F> AsRef<str> for FormStr<F> {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<F> Debug for FormStr<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<F: Default + FormatDefault> Default for FormStr<F> {
    fn default() -> Self {
        Self("".into(), F::default())
    }
}

impl<F> Deref for FormStr<F> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F> Display for FormStr<F> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<F> Eq for FormStr<F> {}

impl<'a, F> From<&'a FormStr<F>> for Cow<'a, str> {
    fn from(value: &'a FormStr<F>) -> Self {
        Cow::Borrowed(&value.0)
    }
}

impl<'a, F> From<FormStr<F>> for Cow<'a, str> {
    fn from(value: FormStr<F>) -> Self {
        Cow::Owned(value.0.into_string())
    }
}

macro_rules! from {
    ($t:ty) => {
        impl<'a, F: Default + Format> TryFrom<$t> for FormStr<F> {
            type Error = FormatErr;

            #[inline]
            fn try_from(value: $t) -> Result<Self> {
                FormStr::new(&value)
            }
        }
    };
}

from!(&Box<str>);
from!(&Cow<'a, str>);
from!(&std::rc::Rc<str>);
from!(&std::sync::Arc<str>);
from!(&str);
from!(Box<str>);
from!(Cow<'a, str>);
from!(std::rc::Rc<str>);
from!(std::sync::Arc<str>);

impl<F> Hash for FormStr<F> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<F> Ord for FormStr<F> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<F> PartialEq for FormStr<F> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<F> PartialOrd for FormStr<F> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

macro_rules! to {
    ($t:ty) => {
        impl<F> From<FormStr<F>> for $t {
            #[inline]
            fn from(f: FormStr<F>) -> $t {
                f.0.into()
            }
        }

        impl<F> From<&FormStr<F>> for $t {
            fn from(f: &FormStr<F>) -> $t {
                f.0.clone().into()
            }
        }
    };
}

to!(std::sync::Arc<str>);
to!(std::rc::Rc<str>);
to!(Box<str>);
to!(String);

#[cfg(feature = "serde")]
impl<'de, F: Default + Format> serde::Deserialize<'de> for FormStr<F> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        FormStr::new(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl<F: Format> serde::Serialize for FormStr<F> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

/// Define a format for use with [FormStr].
pub trait Format {
    fn format<'a>(&self, s: Cow<'a, str>) -> Result<Cow<'a, str>>;
}

impl Format for () {
    fn format<'a>(&self, s: Cow<'a, str>) -> Result<Cow<'a, str>> {
        Ok(s)
    }
}

/// A marker trait that allow a [FormStr] to contains "".
pub trait FormatDefault {}

impl FormatDefault for () {}

/// Errors that can occur during formating.
#[derive(Clone, Copy, PartialEq)]
pub enum FormatErr {
    Fs(FsError),
    MaxLen(usize),
    MinLen,
}

impl Debug for FormatErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fs(e) => Debug::fmt(e, f),
            Self::MaxLen(l) => write!(f, "max len {l}"),
            Self::MinLen => f.write_str("min len"),
        }
    }
}

impl Display for FormatErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for FormatErr {}

pub mod formats {
    use super::*;
    use crate::fs::format_sub_path;

    #[derive(Clone, Copy, Default)]
    pub struct Filename;

    impl Format for Filename {
        fn format<'a>(&self, s: Cow<'a, str>) -> Result<Cow<'a, str>> {
            match s {
                Cow::Borrowed(s) => match validate_filename(s) {
                    Ok(v) => Ok(Cow::Borrowed(v)),
                    Err(e) => Err(FormatErr::Fs(e)),
                },
                Cow::Owned(s) => match validate_filename(&s) {
                    // try to avoid allocating a new string if the string is already trimmed.
                    Ok(v) => Ok(if v.len() == s.len() {
                        Cow::Owned(s)
                    } else {
                        Cow::Owned(v.to_string())
                    }),
                    Err(e) => Err(FormatErr::Fs(e)),
                },
            }
        }
    }

    impl FormatDefault for Filename {}

    /// Makes every chars lower.
    #[derive(Clone, Copy, Default)]
    pub struct Lower<F>(pub F);

    impl<F: Format> Format for Lower<F> {
        fn format<'a>(&self, s: Cow<'a, str>) -> Result<Cow<'a, str>> {
            Ok(Cow::Owned(self.0.format(s)?.to_lowercase()))
        }
    }

    impl<F: FormatDefault> FormatDefault for Lower<F> {}

    /// Enforce a maximum length, returns an error on overflow.
    #[derive(Clone, Copy, Default)]
    pub struct MaxLen<const N: usize, F>(pub F);

    impl<const N: usize, F: Format> Format for MaxLen<N, F> {
        fn format<'a>(&self, s: Cow<'a, str>) -> Result<Cow<'a, str>> {
            let s = self.0.format(s)?;

            if s.len() < N {
                return Ok(s);
            }

            let count = s.chars().take(N + 1).count();

            if count > N {
                Err(FormatErr::MaxLen(N))
            } else {
                Ok(s)
            }
        }
    }

    impl<const N: usize, F: FormatDefault> FormatDefault for MaxLen<N, F> {}

    /// Enforce a minimum length, returns an error if not matching.
    #[derive(Clone, Copy, Default)]
    pub struct MinLen<const N: usize, F>(pub F);

    impl<const N: usize, F: Format> Format for MinLen<N, F> {
        fn format<'a>(&self, s: Cow<'a, str>) -> Result<Cow<'a, str>> {
            let s = self.0.format(s)?;
            let count = s.chars().take(N).count();

            if count >= N {
                Ok(s)
            } else {
                Err(FormatErr::MinLen)
            }
        }
    }

    /// Enforce a sub path, such as `sub_dir/text.txt`
    #[derive(Clone, Copy, Default)]
    pub struct SubPath;

    impl Format for SubPath {
        fn format<'a>(&self, s: Cow<'a, str>) -> Result<Cow<'a, str>> {
            match format_sub_path(&s) {
                Ok(s) => Ok(Cow::Owned(s)),
                Err(e) => Err(FormatErr::Fs(e)),
            }
        }
    }

    /// Trim whitespace chars.
    #[derive(Clone, Copy, Default)]
    pub struct Trim<F>(pub F);

    impl<F: Format> Format for Trim<F> {
        fn format<'a>(&self, s: Cow<'a, str>) -> Result<Cow<'a, str>> {
            Ok(match self.0.format(s)? {
                Cow::Borrowed(s) => Cow::Borrowed(s.trim()),
                Cow::Owned(s) => Cow::Owned(s.trim().to_owned()),
            })
        }
    }

    impl<F: FormatDefault> FormatDefault for Trim<F> {}

    /// Makes all chars upper.
    #[derive(Clone, Copy, Default)]
    pub struct Upper<F>(pub F);

    impl<F: Format> Format for Upper<F> {
        fn format<'a>(&self, s: Cow<'a, str>) -> Result<Cow<'a, str>> {
            Ok(Cow::Owned(self.0.format(s)?.to_uppercase()))
        }
    }

    impl<F: FormatDefault> FormatDefault for Upper<F> {}

    #[test]
    fn test_combination() {
        type C = Lower<MaxLen<3, Trim<()>>>;

        assert!(FormStr::<C>::new(" Hello").is_err());
        assert_eq!(&*FormStr::<C>::new(" Hel").unwrap(), "hel");

        type D = Lower<MinLen<1, MaxLen<3, Trim<()>>>>;

        assert!(FormStr::<D>::new(" ").is_err());
        assert_eq!(&*FormStr::<D>::new(" ABC").unwrap(), "abc");
    }

    #[test]
    fn test_lower() {
        type L = Lower<()>;

        assert_eq!(&*FormStr::<L>::new("Hello").unwrap(), "hello");
    }

    #[test]
    fn test_upper() {
        type U = Upper<()>;

        assert_eq!(&*FormStr::<U>::new("Hello").unwrap(), "HELLO");
    }
}

#[cfg(all(feature = "serde", test))]
#[test]
fn deserialize() {
    use formats::Lower;

    let json = r#""HELLO""#;
    assert_eq!(
        &*serde_json::from_str::<FormStr<Lower<()>>>(json).unwrap(),
        "hello"
    );
}
