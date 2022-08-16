use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
};

#[derive(Clone)]
pub struct Lower<'a>(pub Cow<'a, str>);

impl<'a> Debug for Lower<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<'a> Display for Lower<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<'a> Deref for Lower<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a, S> From<S> for Lower<'a>
where
    Cow<'a, str>: From<S>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl<'a> Lower<'a> {
    fn iter<'b>(&'b self) -> impl Iterator<Item = char> + 'b {
        self.0.chars().flat_map(|c| c.to_lowercase())
    }
}

impl<'a> Eq for Lower<'a> {}

impl<'a> Hash for Lower<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iter().for_each(|c| c.hash(state));
    }
}

impl<'a> Ord for Lower<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<'a> PartialEq<&str> for Lower<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl<'a> PartialEq<str> for Lower<'a> {
    fn eq(&self, other: &str) -> bool {
        self.eq(&Lower(other.into()))
    }
}

impl<'a> PartialEq for Lower<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<'a> PartialOrd<&str> for Lower<'a> {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

impl<'a> PartialOrd<str> for Lower<'a> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        self.partial_cmp(&Lower(other.into()))
    }
}

impl<'a> PartialOrd for Lower<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_lower() {
    assert_eq!(Lower::from("AbC"), Lower::from("abc"));
    assert_eq!(Lower::from("AbC"), "abc");
}
