//! Source position tracking.

/// A position in source text (line, column, byte offset).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    /// 1-based line number.
    pub line: usize,
    /// 1-based column number (byte offset within line).
    pub column: usize,
    /// 0-based byte offset from start of file.
    pub offset: usize,
}

impl Position {
    /// Create a new position.
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }

    /// The start of file position (1:1, offset 0).
    pub fn start() -> Self {
        Self::new(1, 1, 0)
    }

    /// Advance by one column.
    pub fn advance_col(self) -> Self {
        Self {
            line: self.line,
            column: self.column + 1,
            offset: self.offset + 1,
        }
    }

    /// Advance by N columns.
    pub fn advance_by(self, n: usize) -> Self {
        Self {
            line: self.line,
            column: self.column + n,
            offset: self.offset + n,
        }
    }

    /// Move to the next line (reset column to 1).
    pub fn new_line(self) -> Self {
        Self {
            line: self.line + 1,
            column: 1,
            offset: self.offset + 1,
        }
    }

    /// Distance in bytes between two positions.
    pub fn byte_distance_to(self, other: Position) -> usize {
        other.offset.saturating_sub(self.offset)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_start() {
        let p = Position::start();
        assert_eq!(p.line, 1);
        assert_eq!(p.column, 1);
        assert_eq!(p.offset, 0);
    }

    #[test]
    fn test_position_advance() {
        let p = Position::start().advance_by(5);
        assert_eq!(p.column, 6);
        assert_eq!(p.offset, 5);
    }

    #[test]
    fn test_position_newline() {
        let p = Position::new(1, 10, 9).new_line();
        assert_eq!(p.line, 2);
        assert_eq!(p.column, 1);
        assert_eq!(p.offset, 10);
    }

    #[test]
    fn test_position_display() {
        let p = Position::new(3, 14, 27);
        assert_eq!(format!("{}", p), "3:14");
    }

    #[test]
    fn test_byte_distance() {
        let a = Position::new(1, 1, 0);
        let b = Position::new(1, 6, 5);
        assert_eq!(a.byte_distance_to(b), 5);
    }

    #[test]
    fn test_position_ordering() {
        let a = Position::new(1, 1, 0);
        let b = Position::new(2, 1, 10);
        assert!(a < b);
    }
}
