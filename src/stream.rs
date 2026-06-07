//! Token stream with position tracking.

use crate::lookahead::Lookahead;
use crate::span::Span;
use crate::token::{Token, TokenKind};

/// A token stream that wraps a vector of tokens with cursor-based navigation.
pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    /// Create a token stream from a vector of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Create an empty stream with just an EOF token.
    pub fn empty() -> Self {
        Self::new(vec![Token::eof(Span::initial())])
    }

    /// Peek at the current token without advancing.
    pub fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or_else(|| {
            static EOF: std::sync::OnceLock<Token> = std::sync::OnceLock::new();
            EOF.get_or_init(|| Token::eof(Span::initial()))
        })
    }

    /// Consume and return the current token, advancing the cursor.
    pub fn consume(&mut self) -> Token {
        let tok = self.peek().clone();
        if !tok.is_eof() {
            self.pos += 1;
        }
        tok
    }

    /// Check if the current token matches the given kind.
    pub fn check(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    /// Consume if the current token matches the given kind.
    pub fn consume_if(&mut self, kind: TokenKind) -> Option<Token> {
        if self.check(kind) {
            Some(self.consume())
        } else {
            None
        }
    }

    /// Expect a token of the given kind, consuming it. Returns None if mismatch.
    pub fn expect(&mut self, kind: TokenKind) -> Result<Token, Token> {
        let tok = self.peek().clone();
        if tok.kind == kind {
            self.pos += 1;
            Ok(tok)
        } else {
            Err(tok)
        }
    }

    /// Current position in the token stream.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Seek to a specific position.
    pub fn seek(&mut self, pos: usize) {
        self.pos = pos.min(self.tokens.len());
    }

    /// Returns true if at EOF.
    pub fn is_eof(&self) -> bool {
        self.peek().is_eof()
    }

    /// Number of tokens (including EOF).
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Returns true if no tokens.
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Create a lookahead from this stream's remaining tokens.
    pub fn lookahead(self) -> Lookahead<std::vec::IntoIter<Token>> {
        let remaining: Vec<Token> = self.tokens[self.pos..].to_vec();
        Lookahead::new(remaining.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_stream() -> TokenStream {
        let s = Span::initial();
        TokenStream::new(vec![
            Token::new(TokenKind::Ident, "x", s),
            Token::new(TokenKind::Operator, "=", s),
            Token::new(TokenKind::IntLiteral, "42", s),
            Token::new(TokenKind::Punctuation, ";", s),
            Token::eof(s),
        ])
    }

    #[test]
    fn test_peek_consume() {
        let mut ts = make_stream();
        assert_eq!(ts.peek().text, "x");
        let tok = ts.consume();
        assert_eq!(tok.text, "x");
        assert_eq!(ts.peek().text, "=");
    }

    #[test]
    fn test_check() {
        let ts = make_stream();
        assert!(ts.check(TokenKind::Ident));
        assert!(!ts.check(TokenKind::Keyword));
    }

    #[test]
    fn test_consume_if() {
        let mut ts = make_stream();
        let tok = ts.consume_if(TokenKind::Ident);
        assert!(tok.is_some());
        assert_eq!(ts.peek().text, "=");
    }

    #[test]
    fn test_consume_if_mismatch() {
        let mut ts = make_stream();
        let tok = ts.consume_if(TokenKind::Keyword);
        assert!(tok.is_none());
        assert_eq!(ts.peek().text, "x");
    }

    #[test]
    fn test_expect() {
        let mut ts = make_stream();
        let result = ts.expect(TokenKind::Ident);
        assert!(result.is_ok());
    }

    #[test]
    fn test_expect_mismatch() {
        let mut ts = make_stream();
        let result = ts.expect(TokenKind::Keyword);
        assert!(result.is_err());
    }

    #[test]
    fn test_seek() {
        let mut ts = make_stream();
        ts.seek(3);
        assert_eq!(ts.peek().text, ";");
    }

    #[test]
    fn test_eof() {
        let mut ts = make_stream();
        ts.seek(4);
        assert!(ts.is_eof());
        // consuming EOF stays at EOF
        ts.consume();
        assert!(ts.is_eof());
    }

    #[test]
    fn test_empty_stream() {
        let ts = TokenStream::empty();
        assert!(ts.is_eof());
        assert_eq!(ts.len(), 1);
    }

    #[test]
    fn test_position_tracking() {
        let mut ts = make_stream();
        assert_eq!(ts.position(), 0);
        ts.consume();
        assert_eq!(ts.position(), 1);
    }
}
