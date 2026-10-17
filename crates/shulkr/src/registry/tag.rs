use crate::util::{HashMap, HashSet, Key};
use serde::{Deserialize, Deserializer, de::Error as _};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagEntry {
    id: Key,
    tag: bool,
    required: bool,
}

impl TagEntry {
    pub fn element(id: Key) -> Self {
        Self {
            id,
            tag: false,
            required: true,
        }
    }

    pub fn optional_element(id: Key) -> Self {
        Self {
            id,
            tag: false,
            required: false,
        }
    }

    pub fn tag(id: Key) -> Self {
        Self {
            id,
            tag: true,
            required: true,
        }
    }

    pub fn optional_tag(id: Key) -> Self {
        Self {
            id,
            tag: true,
            required: false,
        }
    }

    pub fn id(&self) -> &Key {
        &self.id
    }

    pub fn is_tag(&self) -> bool {
        self.tag
    }

    pub fn is_required(&self) -> bool {
        self.required
    }

    fn parse(raw: String, required: bool) -> Result<Self, String> {
        let (tag, raw) = match raw.strip_prefix('#') {
            Some(stripped) => (true, stripped.to_owned()),
            None => (false, raw),
        };
        let id = Key::try_from(raw).map_err(|error| error.to_string())?;
        Ok(Self { id, tag, required })
    }
}

impl fmt::Display for TagEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tag {
            f.write_str("#")?;
        }
        write!(f, "{}", self.id)?;
        if !self.required {
            f.write_str("?")?;
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for TagEntry {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Raw {
            Id(String),
            Full {
                id: String,
                #[serde(default = "required_default")]
                required: bool,
            },
        }

        fn required_default() -> bool {
            true
        }

        match Raw::deserialize(deserializer)? {
            Raw::Id(id) => Self::parse(id, true),
            Raw::Full { id, required } => Self::parse(id, required),
        }
        .map_err(D::Error::custom)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TagFile {
    #[serde(rename = "values")]
    entries: Vec<TagEntry>,
    #[serde(default)]
    replace: bool,
}

impl TagFile {
    pub fn new(entries: Vec<TagEntry>, replace: bool) -> Self {
        Self { entries, replace }
    }

    pub fn entries(&self) -> &[TagEntry] {
        &self.entries
    }

    pub fn replace(&self) -> bool {
        self.replace
    }
}

#[derive(Debug, Clone, Default)]
pub struct TagLoader {
    builders: HashMap<Key, Vec<TagEntry>>,
}

impl TagLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, id: Key, file: TagFile) {
        let entries = self.builders.entry(id).or_default();
        if file.replace {
            entries.clear();
        }
        entries.extend(file.entries);
    }

    pub fn build(&self, element_exists: impl Fn(&Key) -> bool) -> Tags {
        let mut built: HashMap<Key, Option<Tag>> = HashMap::default();
        let mut visiting = HashSet::default();
        for id in self.builders.keys() {
            self.build_tag(id, &element_exists, &mut visiting, &mut built);
        }

        Tags {
            tags: built
                .into_iter()
                .filter_map(|(id, tag)| tag.map(|tag| (id, tag)))
                .collect(),
        }
    }

    fn build_tag(
        &self,
        id: &Key,
        element_exists: &impl Fn(&Key) -> bool,
        visiting: &mut HashSet<Key>,
        built: &mut HashMap<Key, Option<Tag>>,
    ) {
        if built.contains_key(id) || !visiting.insert(id.clone()) {
            return;
        }
        let Some(entries) = self.builders.get(id) else {
            visiting.remove(id);
            return;
        };

        for entry in entries.iter().filter(|entry| entry.tag) {
            self.build_tag(&entry.id, element_exists, visiting, built);
        }

        let mut ordered = Vec::new();
        let mut seen = HashSet::default();
        let mut missing = Vec::new();
        for entry in entries {
            if entry.tag {
                match built.get(&entry.id) {
                    Some(Some(tag)) => {
                        for key in tag.keys() {
                            if seen.insert(key.clone()) {
                                ordered.push(key.clone());
                            }
                        }
                    }
                    _ if entry.required => missing.push(entry),
                    _ => {}
                }
            } else if element_exists(&entry.id) {
                if seen.insert(entry.id.clone()) {
                    ordered.push(entry.id.clone());
                }
            } else if entry.required {
                missing.push(entry);
            }
        }

        visiting.remove(id);
        if missing.is_empty() {
            built.insert(id.clone(), Some(Tag::new(ordered)));
        } else {
            let missing = missing
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");
            tracing::error!(
                "Couldn't load tag {id} as it is missing following references: {missing}"
            );
            built.insert(id.clone(), None);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Tag {
    ordered: Vec<Key>,
    members: HashSet<Key>,
}

impl Tag {
    pub fn new(ordered: Vec<Key>) -> Self {
        let members = ordered.iter().cloned().collect();
        Self { ordered, members }
    }

    pub fn contains(&self, key: &Key) -> bool {
        self.members.contains(key)
    }

    pub fn keys(&self) -> &[Key] {
        &self.ordered
    }

    pub fn is_empty(&self) -> bool {
        self.ordered.is_empty()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Tags {
    tags: HashMap<Key, Tag>,
}

impl Tags {
    pub fn insert(&mut self, tag: Key, ordered: Vec<Key>) {
        self.tags.insert(tag, Tag::new(ordered));
    }

    pub fn remove(&mut self, tag: &Key) -> Option<Tag> {
        self.tags.remove(tag)
    }

    pub fn contains(&self, tag: &Key, key: &Key) -> bool {
        self.tags.get(tag).is_some_and(|tag| tag.contains(key))
    }

    pub fn get(&self, tag: &Key) -> Option<&Tag> {
        self.tags.get(tag)
    }

    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.tags.keys()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Tag)> {
        self.tags.iter()
    }

    pub fn len(&self) -> usize {
        self.tags.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(json: &str) -> TagFile {
        serde_json::from_str(json).unwrap()
    }

    fn key(path: &'static str) -> Key {
        Key::const_vanilla(path)
    }

    #[test]
    fn references_resolve_in_order_without_duplicates() {
        let mut loader = TagLoader::new();
        loader.add(
            key("logs"),
            file(r##"{"values": ["#minecraft:oak", "minecraft:stem", "minecraft:oak_log"]}"##),
        );
        loader.add(
            key("oak"),
            file(r#"{"values": ["minecraft:oak_log", "minecraft:oak_wood"]}"#),
        );

        let tags = loader.build(|_| true);
        assert_eq!(
            tags.get(&key("logs")).unwrap().keys(),
            [key("oak_log"), key("oak_wood"), key("stem")]
        );
    }

    #[test]
    fn files_merge_unless_replace() {
        let mut loader = TagLoader::new();
        loader.add(key("a"), file(r#"{"values": ["minecraft:x"]}"#));
        loader.add(key("a"), file(r#"{"values": ["minecraft:y"]}"#));
        loader.add(key("b"), file(r#"{"values": ["minecraft:x"]}"#));
        loader.add(
            key("b"),
            file(r#"{"replace": true, "values": ["minecraft:y"]}"#),
        );

        let tags = loader.build(|_| true);
        assert_eq!(tags.get(&key("a")).unwrap().keys(), [key("x"), key("y")]);
        assert_eq!(tags.get(&key("b")).unwrap().keys(), [key("y")]);
    }

    #[test]
    fn missing_required_entries_drop_the_tag() {
        let mut loader = TagLoader::new();
        loader.add(
            key("element"),
            file(r#"{"values": ["minecraft:x", "minecraft:unknown"]}"#),
        );
        loader.add(
            key("tag"),
            file(r##"{"values": ["minecraft:x", "#minecraft:unknown"]}"##),
        );
        loader.add(
            key("dependent"),
            file(r##"{"values": ["#minecraft:element"]}"##),
        );

        let tags = loader.build(|id| id == &key("x"));
        assert!(tags.get(&key("element")).is_none());
        assert!(tags.get(&key("tag")).is_none());
        assert!(tags.get(&key("dependent")).is_none());
    }

    #[test]
    fn missing_optional_entries_are_skipped() {
        let mut loader = TagLoader::new();
        loader.add(
            key("a"),
            file(
                r##"{"values": ["minecraft:x", {"id": "minecraft:unknown", "required": false}, {"id": "#minecraft:unknown", "required": false}]}"##,
            ),
        );

        let tags = loader.build(|id| id == &key("x"));
        assert_eq!(tags.get(&key("a")).unwrap().keys(), [key("x")]);
    }

    #[test]
    fn cycles_do_not_hang() {
        let mut loader = TagLoader::new();
        loader.add(key("a"), file(r##"{"values": ["#minecraft:b"]}"##));
        loader.add(key("b"), file(r##"{"values": ["#minecraft:a"]}"##));

        let tags = loader.build(|_| true);
        assert!(tags.get(&key("a")).is_none() || tags.get(&key("b")).is_none());
    }
}
