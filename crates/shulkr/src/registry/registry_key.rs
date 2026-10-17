use crate::util::Key;
use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
};

#[derive(Debug)]
pub struct RegistryKey<T> {
    key: Key,
    _phantom: PhantomData<T>,
}

impl<T> RegistryKey<T> {
    pub fn new(path: String) -> Self {
        Self {
            key: Key::vanilla(path),
            _phantom: PhantomData,
        }
    }

    pub fn of(path: String) -> Self {
        Self {
            key: Key::of(path),
            _phantom: PhantomData,
        }
    }

    pub const fn const_new(namespace: &'static str, path: &'static str) -> Self {
        Self {
            key: Key::const_new(namespace, path),
            _phantom: PhantomData,
        }
    }

    pub const fn const_vanilla(path: &'static str) -> Self {
        Self {
            key: Key::const_vanilla(path),
            _phantom: PhantomData,
        }
    }

    pub fn to_key(&self) -> Key {
        self.key.clone()
    }
}

impl<T> PartialEq for RegistryKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for RegistryKey<T> {}

impl<T> Hash for RegistryKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<T> From<Key> for RegistryKey<T> {
    fn from(value: Key) -> Self {
        Self {
            key: value,
            _phantom: PhantomData,
        }
    }
}

impl<T> From<RegistryKey<T>> for Key {
    fn from(value: RegistryKey<T>) -> Self {
        value.to_key()
    }
}

impl<T> Deref for RegistryKey<T> {
    type Target = Key;

    fn deref(&self) -> &Self::Target {
        &self.key
    }
}

impl<T> Clone for RegistryKey<T> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            _phantom: PhantomData,
        }
    }
}
