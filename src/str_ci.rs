use crate::{cmp::EqExt, form_str::FormStr};
use std::{
    borrow::Borrow,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
};

/// A string reference that perform case insentive comparison. It can be put into an HashMap.
/// For an owned version, see [StringCi].
#[repr(transparent)]
pub struct StrCi(str);

impl StrCi {
    #[inline]
    pub fn new(s: &str) -> &Self {
        unsafe { std::mem::transmute(s) }
    }
}

impl AsRef<str> for StrCi {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Debug for StrCi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Deref for StrCi {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for StrCi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Eq for StrCi {}

impl Hash for StrCi {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash_ci(state);
    }
}

impl<F> PartialEq<FormStr<F>> for StrCi {
    #[inline]
    fn eq(&self, other: &FormStr<F>) -> bool {
        self.0.eq_ci(other)
    }
}

impl PartialEq<str> for StrCi {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.0.eq_ci(other)
    }
}

impl PartialEq for StrCi {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ci(&other.0)
    }
}

impl ToOwned for StrCi {
    type Owned = StringCi;

    fn to_owned(&self) -> Self::Owned {
        StringCi(self.0.to_string())
    }
}

/// An owned string that perform case insentive comparison. It can be put into an HashMap.
/// For a reference string, see [StrCi].
#[derive(Clone, Default)]
pub struct StringCi(pub String);

impl StringCi {
    #[inline]
    pub fn as_str_ci(&self) -> &StrCi {
        let s = self.0.as_str();
        unsafe { std::mem::transmute(s) }
    }
}

impl AsRef<str> for StringCi {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<StrCi> for StringCi {
    #[inline]
    fn as_ref(&self) -> &StrCi {
        StrCi::new(self.0.as_ref())
    }
}

impl Borrow<StrCi> for StringCi {
    #[inline]
    fn borrow(&self) -> &StrCi {
        self.as_str_ci()
    }
}

impl Debug for StringCi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for StringCi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Eq for StringCi {}

impl Hash for StringCi {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash_ci(state);
    }
}

impl PartialEq for StringCi {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.deref().eq_ci(other.0.deref())
    }
}
