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
        self.start.saturating_add(self.length)
    }

    /// Returns `true` when the range is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.length == 0
    }

    #[must_use]
    pub fn byte_range(self, text: &str) -> Option<core::ops::Range<usize>> {
        let end = self.start.checked_add(self.length)?;
        let mut start_byte = None;
        let mut utf16_offset = 0;
        for (byte_offset, character) in text.char_indices() {
            if utf16_offset == self.start {
                start_byte = Some(byte_offset);
            }
            if utf16_offset == end {
                return Some(start_byte?..byte_offset);
            }
            utf16_offset += character.len_utf16();
        }
        if utf16_offset == self.start {
            start_byte = Some(text.len());
        }
        if utf16_offset == end {
            Some(start_byte?..text.len())
        } else {
            None
        }
    }
}

impl From<(usize, usize)> for TextRange {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[cfg(test)]
mod tests {
    use super::TextRange;

    #[test]
    fn byte_range_maps_ascii_offsets() {
        assert_eq!(TextRange::new(6, 5).byte_range("Hello world"), Some(6..11));
        assert_eq!(TextRange::new(0, 0).byte_range(""), Some(0..0));
    }

    #[test]
    fn byte_range_maps_multi_unit_characters() {
        let text = "a\u{1F44B}\u{1F3FD}b caf\u{e9}";
        assert_eq!(TextRange::new(1, 4).byte_range(text), Some(1..9));
        assert_eq!(&text[1..9], "\u{1F44B}\u{1F3FD}");
        assert_eq!(TextRange::new(10, 1).byte_range(text), Some(14..16));
        assert_eq!(&text[14..16], "\u{e9}");
        assert_eq!(TextRange::new(11, 0).byte_range(text), Some(16..16));
    }

    #[test]
    fn byte_range_rejects_split_surrogates_and_out_of_bounds() {
        let text = "a\u{1F44B}b";
        assert_eq!(TextRange::new(2, 1).byte_range(text), None);
        assert_eq!(TextRange::new(0, 2).byte_range(text), None);
        assert_eq!(TextRange::new(0, 5).byte_range(text), None);
        assert_eq!(TextRange::new(5, 0).byte_range(text), None);
        assert_eq!(TextRange::new(1, usize::MAX).byte_range(text), None);
    }
}
