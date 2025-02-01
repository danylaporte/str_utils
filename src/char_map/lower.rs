use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
};

#[derive(Clone)]
pub struct Lower<'a>(pub Cow<'a, str>);

impl Debug for Lower<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Lower<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Deref for Lower<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
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

impl Lower<'_> {
    fn iter(&self) -> impl Iterator<Item = char> + '_ {
        self.0.chars().flat_map(|c| c.to_lowercase())
    }
}

impl Eq for Lower<'_> {}

impl Hash for Lower<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.iter().for_each(|c| c.hash(state));
    }
}

impl Ord for Lower<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl PartialEq<&str> for Lower<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<str> for Lower<'_> {
    fn eq(&self, other: &str) -> bool {
        self.eq(&Lower(other.into()))
    }
}

impl PartialEq for Lower<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl PartialOrd<&str> for Lower<'_> {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

impl PartialOrd<str> for Lower<'_> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        self.partial_cmp(&Lower(other.into()))
    }
}

impl PartialOrd for Lower<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_lower() {
    assert_eq!(Lower::from("AbC"), Lower::from("abc"));
    assert_eq!(Lower::from("AbC"), "abc");
}
