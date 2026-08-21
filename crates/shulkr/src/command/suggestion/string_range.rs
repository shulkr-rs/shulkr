#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringRange {
    start: usize,
    end: usize,
}

impl StringRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn at(pos: usize) -> Self {
        Self::new(pos, pos)
    }

    pub fn between(start: usize, end: usize) -> Self {
        Self::new(start, end)
    }

    pub fn encompassing(a: StringRange, b: StringRange) -> Self {
        Self::new(a.start.min(b.start), a.end.max(b.end))
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }

    pub fn get<'a>(&self, input: &'a str) -> &'a str {
        let start = self.start.min(input.len());
        let end = self.end.min(input.len()).max(start);
        &input[start..end]
    }
}

impl Default for StringRange {
    fn default() -> Self {
        Self::at(0)
    }
}
