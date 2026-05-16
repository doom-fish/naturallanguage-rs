//! Shared `NaturalLanguage` value types.

/// A UTF-16 text range, matching Apple's `NSRange`-based `NaturalLanguage` APIs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TextRange {
    /// UTF-16 start offset.
    pub start: usize,
    /// UTF-16 length.
    pub length: usize,
}

impl TextRange {
    /// Create a new range.
    #[must_use]
    pub const fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    /// UTF-16 end offset.
    #[must_use]
    pub const fn end(self) -> usize {
        self.start + self.length
    }

    /// Returns `true` when the range is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.length == 0
    }
}

impl From<(usize, usize)> for TextRange {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}
