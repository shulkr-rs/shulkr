use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Key {
    namespace: Cow<'static, str>,
    path: Cow<'static, str>,
}

impl Key {
    pub fn new<A, B>(namespace: A, path: B) -> Self
    where
        A: Into<Cow<'static, str>>,
        B: Into<Cow<'static, str>>,
    {
        Self {
            namespace: namespace.into(),
            path: path.into(),
        }
    }

    pub const fn const_new(namespace: &'static str, path: &'static str) -> Self {
        Self {
            namespace: Cow::Borrowed(namespace),
            path: Cow::Borrowed(path),
        }
    }

    pub fn vanilla<S>(path: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self::new("minecraft", path)
    }

    pub const fn const_vanilla(path: &'static str) -> Self {
        Self::const_new("minecraft", path)
    }

    pub fn of<S>(key: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        let key = key.into();
        if let Some((namespace, path)) = key.split_once(':') {
            Self::new(namespace.to_string(), path.to_string())
        } else {
            Self::vanilla(key)
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl<S> From<S> for Key
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self::of(value.into())
    }
}
