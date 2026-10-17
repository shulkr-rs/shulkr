use std::ops::Deref;

use super::Id;
use crate::util::Key;

#[derive(Debug)]
pub struct Ref<T> {
    key: Key,
    id: Id,
    value: T,
}

impl<T> Ref<T> {
    pub fn new(key: Key, id: Id, value: T) -> Self {
        Self { key, id, value }
    }

    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> Deref for Ref<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> PartialEq for Ref<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for Ref<T> {}
