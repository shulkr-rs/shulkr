use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(try_from = "String")]
pub struct Key {
    namespace: Cow<'static, str>,
    path: Cow<'static, str>,
}

impl Key {
    pub const VANILLA_NAMESPACE: &'static str = "minecraft";

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
        Self::new(Self::VANILLA_NAMESPACE, path)
    }

    pub const fn const_vanilla(path: &'static str) -> Self {
        Self::const_new(Self::VANILLA_NAMESPACE, path)
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

    pub const fn valid_namespace_char(char: char) -> bool {
        char == '_'
            || char == '-'
            || char.is_ascii_lowercase()
            || char.is_ascii_digit()
            || char == '.'
    }

    pub const fn valid_char(char: char) -> bool {
        Self::valid_namespace_char(char) || char == '/'
    }

    pub fn validate_namespace(namespace: &str) -> bool {
        namespace != ".." && namespace.chars().all(Self::valid_namespace_char)
    }

    pub fn validate_path(path: &str) -> bool {
        path.chars().all(Self::valid_char)
    }

    pub fn validate(namespace: &str, path: &str) -> bool {
        Self::validate_namespace(namespace) && Self::validate_path(path)
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidKey {
    Namespace(String),
    Path(String),
}

impl std::fmt::Display for InvalidKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Namespace(namespace) => write!(f, "invalid namespace: {namespace}"),
            Self::Path(path) => write!(f, "invalid path: {path}"),
        }
    }
}

impl std::error::Error for InvalidKey {}

impl From<&'static str> for Key {
    fn from(value: &'static str) -> Self {
        Self::of(value)
    }
}

impl TryFrom<String> for Key {
    type Error = InvalidKey;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (namespace, path) = match value.split_once(':') {
            Some(("", path)) => (Key::VANILLA_NAMESPACE, path),
            Some((namespace, path)) => (namespace, path),
            None => (Key::VANILLA_NAMESPACE, value.as_str()),
        };

        if !Key::validate_namespace(namespace) {
            return Err(InvalidKey::Namespace(namespace.to_owned()));
        }

        if !Key::validate_path(path) {
            return Err(InvalidKey::Path(path.to_owned()));
        }

        Ok(Key::new(namespace.to_owned(), path.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_parses_namespaced_and_bare_paths() {
        assert_eq!(
            Key::try_from(String::from("minecraft:stone")).expect("test value"),
            Key::new("minecraft", "stone")
        );
        assert_eq!(
            Key::try_from(String::from("stone")).expect("test value"),
            Key::new("minecraft", "stone")
        );
        assert_eq!(
            Key::try_from(String::from(":stone")).expect("test value"),
            Key::new("minecraft", "stone")
        );
    }

    #[test]
    fn try_from_rejects_invalid_namespace_and_path() {
        assert!(matches!(
            Key::try_from(String::from("Minecraft:stone")),
            Err(InvalidKey::Namespace(_))
        ));
        assert!(matches!(
            Key::try_from(String::from("..:stone")),
            Err(InvalidKey::Namespace(_))
        ));
        assert!(matches!(
            Key::try_from(String::from("minecraft:Stone")),
            Err(InvalidKey::Path(_))
        ));
    }

    #[test]
    fn display_roundtrips_through_try_from() {
        let key = Key::new("modid", "some/nested/path");
        let text = key.to_string();
        assert_eq!(text, "modid:some/nested/path");
        assert_eq!(Key::try_from(text).expect("test value"), key);
    }

    #[test]
    fn from_static_str_is_infallible_and_defaults_namespace() {
        assert_eq!(Key::from("stone"), Key::vanilla("stone"));
        assert_eq!(Key::from("modid:stone"), Key::new("modid", "stone"));
    }

    #[test]
    fn deserializes_through_try_from() {
        let key: Key = serde_json::from_str(r#""modid:some/path""#).expect("test value");
        assert_eq!(key, Key::new("modid", "some/path"));
        assert!(serde_json::from_str::<Key>(r#""Minecraft:Stone""#).is_err());
    }
}
