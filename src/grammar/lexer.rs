//! A tokenizer for ebnf grammar files.
//! The tokens produced are:
//!     - Ident       - a non-terminal
//!     - Term        - a terminal
//!     - LParen      - '('
//!     - RParen      - ')'
//!     - LBracket    - '['
//!     - RBracket    - ']'
//!     - LBrace      - '{'
//!     - RBrace      - '}'
//!     - Pipe        - '|'
//!     - Colon       - ':'
//!     - SemiColon   - ';'


use super::token::{Token, TokenKind};

pub struct Lexer<'a> {
    text: &'a str,
    pos: usize
}

impl<'a> Lexer<'a> {
    const LITERALS: &'static [(&'static str, TokenKind)] = &[
        ("(", TokenKind::LParen),
        (")", TokenKind::RParen),
        ("[", TokenKind::LBracket),
        ("]", TokenKind::RBracket),
        ("{", TokenKind::LBrace),
        ("}", TokenKind::RBrace),
        ("|", TokenKind::Pipe),
        (":", TokenKind::Colon),
        (";", TokenKind::SemiColon)
    ];

    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            pos: 0
        }
    }

    fn consume_next(&mut self) -> Option<&'a str> {
        let start = self.pos;
        let next = self.text.chars().next()?;
        self.pos += next.len_utf8();
        Some(&self.text[start..self.pos])
    }

    fn consume_str(&mut self, s: &str) -> Option<&'a str> {
        let start = self.pos;
        if self.text[self.pos..].starts_with(s) {
            self.pos += s.len();
            Some(&self.text[start..self.pos])
        } else {
            None
        }
    }

    fn consume_if(&mut self, f: impl FnOnce(char) -> bool) -> Option<&'a str> {
        let start = self.pos;
        let next = self.text.chars().next()?;
        if f(next) {
            self.pos += next.len_utf8();
            Some(&self.text[start..self.pos])
        } else {
            None
        }
    }

    fn consume_while(&mut self, mut f: impl FnMut(char) -> bool) -> Option<&'a str> {
        let start = self.pos;
        while let Some(chr) = self.text[self.pos..].chars().next() {
            if f(chr) {
                self.pos += chr.len_utf8();
            } else {
                break;
            }
        }
        if self.pos != start {
            Some(&self.text[start..self.pos])
        } else {
            None
        }
    }

    fn terminal(&mut self) -> Option<String> {
        let mut id = String::new();
        self.consume_str("'")?;
        while let Some(chr) = self.consume_next() {
            if chr == "'" {
                return Some(id);
            } else if chr == "\\" {
                let chr = self.consume_next()?;
                match chr {
                    "t" => id.push('\t'),
                    "n" => id.push('\n'),
                    "r" => id.push('\r'),
                    "0" => id.push('\0'),
                    "'" => id.push('\''),
                    "\\" => id.push('\\'),
                    _ => return None
                }
            } else {
                id.push_str(chr);
            }
        }
        None
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        self.consume_while(char::is_whitespace);
        if self.pos >= self.text.len() {
            return None;
        }
        let start = self.pos;
        for (lit, kind) in Self::LITERALS {
            if let Some(t) = self.consume_str(lit) {
                return Some(Token {
                    kind: kind.clone(),
                    text: t,
                    pos: start
                });
            }
        }

        if let Some(t) = self.consume_while(|c| c.is_alphanumeric() || c == '_') {
            return Some(Token {
                kind: TokenKind::Ident(String::from(t)),
                text: t,
                pos: start
            });
        }

        if let Some(t) = self.terminal() {
            return Some(Token {
                kind: TokenKind::Term(t),
                text: &self.text[start..self.pos],
                pos: start
            });
        }

        None
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        self.next_token()
    }
}
