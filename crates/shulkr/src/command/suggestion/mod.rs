mod builder;
mod string_range;
mod suggestions;

pub use builder::SuggestionsBuilder;
pub use string_range::StringRange;
pub use suggestions::Suggestions;

use crate::text::TextComponent;

#[derive(Debug, Clone, PartialEq)]
pub struct Suggestion {
    range: StringRange,
    text: String,
    tooltip: Option<TextComponent>,
}

impl Suggestion {
    pub fn new(range: StringRange, text: impl Into<String>) -> Self {
        Self {
            range,
            text: text.into(),
            tooltip: None,
        }
    }

    pub fn with_tooltip(mut self, tooltip: impl Into<TextComponent>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    pub fn range(&self) -> StringRange {
        self.range
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn tooltip(&self) -> Option<&TextComponent> {
        self.tooltip.as_ref()
    }

    pub fn apply(&self, input: &str) -> String {
        if self.range.start() == 0 && self.range.end() == input.len() {
            return self.text.clone();
        }
        let mut result = String::new();
        result.push_str(&input[..self.range.start().min(input.len())]);
        result.push_str(&self.text);
        if self.range.end() < input.len() {
            result.push_str(&input[self.range.end()..]);
        }
        result
    }

    fn expand(&self, input: &str, range: StringRange) -> Suggestion {
        if range == self.range {
            return self.clone();
        }
        let mut text = String::new();
        if range.start() < self.range.start() {
            text.push_str(&input[range.start()..self.range.start().min(input.len())]);
        }
        text.push_str(&self.text);
        if range.end() > self.range.end() {
            text.push_str(&input[self.range.end().min(input.len())..range.end().min(input.len())]);
        }
        Suggestion {
            range,
            text,
            tooltip: self.tooltip.clone(),
        }
    }
}
