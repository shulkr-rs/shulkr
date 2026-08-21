use crate::command::suggestion::{StringRange, Suggestion};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Suggestions {
    range: StringRange,
    list: Vec<Suggestion>,
}
impl Suggestions {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new(range: StringRange, list: Vec<Suggestion>) -> Self {
        Self { range, list }
    }

    pub fn range(&self) -> StringRange {
        self.range
    }

    pub fn list(&self) -> &[Suggestion] {
        &self.list
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn create(input: &str, suggestions: Vec<Suggestion>) -> Self {
        if suggestions.is_empty() {
            return Self::empty();
        }

        let start = suggestions.iter().map(|s| s.range().start()).min().unwrap();
        let end = suggestions.iter().map(|s| s.range().end()).max().unwrap();
        let range = StringRange::new(start, end);

        let mut texts: Vec<Suggestion> = suggestions
            .iter()
            .map(|suggestion| suggestion.expand(input, range))
            .collect();

        texts.sort_by_key(|suggestion| suggestion.text().to_lowercase());
        texts.dedup_by(|a, b| a.text() == b.text() && a.range() == b.range());

        Self::new(range, texts)
    }

    pub fn merge(input: &str, inputs: Vec<Suggestions>) -> Self {
        match inputs.len() {
            0 => Self::empty(),
            1 => inputs.into_iter().next().unwrap(),
            _ => Self::create(input, inputs.into_iter().flat_map(|s| s.list).collect()),
        }
    }
}
