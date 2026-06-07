//! Lookahead buffer for token streams.

use crate::token::{Token, TokenKind};

/// A lookahead buffer wrapping a token iterator.
/// Peeks ahead without consuming tokens.
pub struct Lookahead<I: Iterator<Item = Token>> {
    source: I,
    buffer: Vec<Token>,
    pos: usize,
}

impl<I: Iterator<Item = Token>> Lookahead<I> {
    /// Create a new lookahead buffer from a token iterator.
    pub fn new(source: I) -> Self {
        Self {
            source,
            buffer: Vec::new(),
            pos: 0,
        }
    }

    /// Ensure at least `n` tokens are buffered.
    fn fill_buffer(&mut self, n: usize) {
        while self.buffer.len() < self.pos + n {
            match self.source.next() {
                Some(tok) => self.buffer.push(tok),
                None => break,
            }
        }
    }

    /// Peek at the nth token ahead without consuming (0 = next).
    pub fn peek(&mut self, n: usize) -> Option<&Token> {
        self.fill_buffer(n + 1);
        self.buffer.get(self.pos + n)
    }

    /// Check if the next token has the given kind.
    pub fn peek_is(&mut self, kind: TokenKind) -> bool {
        self.peek(0).is_some_and(|t| t.kind == kind)
    }

    /// Check if the nth token ahead has the given kind.
    pub fn peek_nth_is(&mut self, n: usize, kind: TokenKind) -> bool {
        self.peek(n).is_some_and(|t| t.kind == kind)
    }

    /// Consume and return the next token.
    pub fn consume(&mut self) -> Option<Token> {
        self.fill_buffer(1);
        if self.pos < self.buffer.len() {
            let tok = self.buffer[self.pos].clone();
            self.pos += 1;
            // Compact buffer if we've consumed enough
            if self.pos > 64 {
                self.buffer.drain(..self.pos);
                self.pos = 0;
            }
            Some(tok)
        } else {
            None
        }
    }

    /// Number of tokens consumed so far.
    pub fn consumed(&self) -> usize {
        self.pos
    }

    /// Check if at end of stream.
    pub fn is_eof(&mut self) -> bool {
        self.peek(0).is_none_or(|t| t.is_eof())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::Position;
    use crate::span::Span;

    fn make_tokens() -> Vec<Token> {
        let s = Span::initial();
        vec![
            Token::new(TokenKind::Ident, "a", s),
            Token::new(TokenKind::Operator, "+", s),
            Token::new(TokenKind::Ident, "b", s),
            Token::eof(Span::new(Position::start(), Position::start())),
        ]
    }

    #[test]
    fn test_peek_ahead() {
        let tokens = make_tokens();
        let mut la = Lookahead::new(tokens.into_iter());
        assert_eq!(la.peek(0).unwrap().text, "a");
        assert_eq!(la.peek(1).unwrap().text, "+");
        assert_eq!(la.peek(2).unwrap().text, "b");
    }

    #[test]
    fn test_consume() {
        let tokens = make_tokens();
        let mut la = Lookahead::new(tokens.into_iter());
        let tok = la.consume().unwrap();
        assert_eq!(tok.text, "a");
        assert_eq!(la.peek(0).unwrap().text, "+");
    }

    #[test]
    fn test_peek_is() {
        let tokens = make_tokens();
        let mut la = Lookahead::new(tokens.into_iter());
        assert!(la.peek_is(TokenKind::Ident));
        assert!(!la.peek_is(TokenKind::Operator));
    }

    #[test]
    fn test_is_eof() {
        let tokens = make_tokens();
        let mut la = Lookahead::new(tokens.into_iter());
        assert!(!la.is_eof());
        la.consume();
        la.consume();
        la.consume();
        assert!(la.is_eof());
    }

    #[test]
    fn test_consumed_count() {
        let tokens = make_tokens();
        let mut la = Lookahead::new(tokens.into_iter());
        assert_eq!(la.consumed(), 0);
        la.consume();
        assert_eq!(la.consumed(), 1);
    }
}
