//! Token types and definitions.

use crate::span::Span;

/// Kinds of lexical tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenKind {
    /// Identifier.
    Ident,
    /// Integer literal.
    IntLiteral,
    /// String literal.
    StringLiteral,
    /// Operator (+, -, *, /, etc.).
    Operator,
    /// Punctuation (parentheses, braces, commas, etc.).
    Punctuation,
    /// Keyword.
    Keyword,
    /// End of file.
    Eof,
    /// Unknown/invalid token.
    Unknown,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Ident => write!(f, "ident"),
            TokenKind::IntLiteral => write!(f, "int"),
            TokenKind::StringLiteral => write!(f, "string"),
            TokenKind::Operator => write!(f, "op"),
            TokenKind::Punctuation => write!(f, "punct"),
            TokenKind::Keyword => write!(f, "keyword"),
            TokenKind::Eof => write!(f, "eof"),
            TokenKind::Unknown => write!(f, "unknown"),
        }
    }
}

/// A single lexical token with its kind, text, and source span.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: Span,
}

impl Token {
    /// Create a new token.
    pub fn new(kind: TokenKind, text: impl Into<String>, span: Span) -> Self {
        Self {
            kind,
            text: text.into(),
            span,
        }
    }

    /// Create an EOF token at the given position.
    pub fn eof(span: Span) -> Self {
        Self::new(TokenKind::Eof, "", span)
    }

    /// Returns true if this is an EOF token.
    pub fn is_eof(&self) -> bool {
        self.kind == TokenKind::Eof
    }

    /// Check if this token has a specific kind.
    pub fn is_kind(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }

    /// Returns the text length.
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Returns true if the token text is empty.
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(\"{}\")", self.kind, self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;

    #[test]
    fn test_token_creation() {
        let span = Span::new(Position::start(), Position::new(1, 4, 3));
        let tok = Token::new(TokenKind::Ident, "foo", span);
        assert_eq!(tok.text, "foo");
        assert_eq!(tok.kind, TokenKind::Ident);
    }

    #[test]
    fn test_eof_token() {
        let span = Span::initial();
        let tok = Token::eof(span);
        assert!(tok.is_eof());
        assert!(tok.is_empty());
    }

    #[test]
    fn test_token_display() {
        let span = Span::initial();
        let tok = Token::new(TokenKind::IntLiteral, "42", span);
        assert_eq!(format!("{}", tok), "int(\"42\")");
    }

    #[test]
    fn test_is_kind() {
        let span = Span::initial();
        let tok = Token::new(TokenKind::Keyword, "fn", span);
        assert!(tok.is_kind(TokenKind::Keyword));
        assert!(!tok.is_kind(TokenKind::Ident));
    }

    #[test]
    fn test_token_len() {
        let span = Span::initial();
        let tok = Token::new(TokenKind::Ident, "hello", span);
        assert_eq!(tok.len(), 5);
    }
}
