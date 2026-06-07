//! Source span tracking.

use crate::position::Position;

/// A span of source text from start to end (inclusive start, exclusive end).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    /// Create a new span.
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    /// A zero-length span at the start of file.
    pub fn initial() -> Self {
        let p = Position::start();
        Self::new(p, p)
    }

    /// Length of the span in bytes.
    pub fn len(&self) -> usize {
        self.start.byte_distance_to(self.end)
    }

    /// Returns true if the span is zero-length.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Merge two spans into one covering both.
    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Check if this span contains a position.
    pub fn contains(&self, pos: Position) -> bool {
        pos >= self.start && pos < self.end
    }

    /// Check if this span overlaps another.
    pub fn overlaps(&self, other: &Span) -> bool {
        self.start < other.end && other.start < self.end
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_len() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 6, 5));
        assert_eq!(span.len(), 5);
    }

    #[test]
    fn test_span_empty() {
        let span = Span::initial();
        assert!(span.is_empty());
    }

    #[test]
    fn test_span_merge() {
        let a = Span::new(Position::new(1, 1, 0), Position::new(1, 5, 4));
        let b = Span::new(Position::new(1, 3, 2), Position::new(1, 10, 9));
        let merged = a.merge(b);
        assert_eq!(merged.start, Position::new(1, 1, 0));
        assert_eq!(merged.end, Position::new(1, 10, 9));
    }

    #[test]
    fn test_span_contains() {
        let span = Span::new(Position::new(1, 1, 0), Position::new(1, 5, 4));
        assert!(span.contains(Position::new(1, 3, 2)));
        assert!(!span.contains(Position::new(1, 5, 4)));
    }

    #[test]
    fn test_span_overlaps() {
        let a = Span::new(Position::new(1, 1, 0), Position::new(1, 5, 4));
        let b = Span::new(Position::new(1, 3, 2), Position::new(1, 8, 7));
        assert!(a.overlaps(&b));
    }
}
