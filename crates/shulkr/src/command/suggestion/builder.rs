use crate::{
    command::suggestion::{StringRange, Suggestion, Suggestions},
    text::TextComponent,
};

#[derive(Debug, Clone)]
pub struct SuggestionsBuilder {
    input: String,
    start: usize,
    remaining: String,
    remaining_lowercase: String,
    result: Vec<Suggestion>,
}

impl SuggestionsBuilder {
    pub fn new(input: impl Into<String>, start: usize) -> Self {
        let input = input.into();
        let remaining = input[start.min(input.len())..].to_string();
        let remaining_lowercase = remaining.to_lowercase();
        Self {
            input,
            start,
            remaining,
            remaining_lowercase,
            result: Vec::new(),
        }
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn remaining(&self) -> &str {
        &self.remaining
    }

    pub fn remaining_lowercase(&self) -> &str {
        &self.remaining_lowercase
    }

    pub fn suggest(&mut self, text: impl Into<String>) -> &mut Self {
        let text = text.into();
        if text != self.remaining {
            self.result.push(Suggestion::new(
                StringRange::between(self.start, self.input.len()),
                text,
            ));
        }
        self
    }

    pub fn suggest_with_tooltip(
        &mut self,
        text: impl Into<String>,
        tooltip: impl Into<TextComponent>,
    ) -> &mut Self {
        let text = text.into();
        if text != self.remaining {
            self.result.push(
                Suggestion::new(StringRange::between(self.start, self.input.len()), text)
                    .with_tooltip(tooltip),
            );
        }
        self
    }

    pub fn suggest_matching<I, T>(&mut self, candidates: I) -> &mut Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        for candidate in candidates {
            let candidate = candidate.into();
            if candidate
                .to_lowercase()
                .starts_with(&self.remaining_lowercase)
            {
                self.suggest(candidate);
            }
        }
        self
    }

    pub fn create_offset(&self, start: usize) -> Self {
        Self::new(self.input.clone(), start)
    }

    pub fn build(self) -> Suggestions {
        Suggestions::create(&self.input, self.result)
    }
}
