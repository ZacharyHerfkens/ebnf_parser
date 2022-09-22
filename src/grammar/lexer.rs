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

use std::str::CharIndices;
use std::iter::Peekable;

use super::token::{Token, TokenKind};
pub struct Tokenizer<'a> {
    text: &'a str,
    chars: Peekable<CharIndices<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            chars: text.char_indices().peekable(),
            line: 1,
            col: 1,
        }
    }
    
    fn cur_char(&self) -> Option<&str> {
        self.chars.peek().map(|&(pos, chr)| {
            self.text.get(pos..pos+chr.len_utf8())
        })
        .flatten()
    }

    fn next_char(&mut self) -> Option<&str> {
        self.chars.next().map(|(pos, chr)| {
            self.text.get(pos..pos+chr.len_utf8())
        })
        .flatten()
    }
    
    fn next_if(&mut self, pred: impl FnOnce(char) -> bool) -> Option<&str> {
        let chr = self.cur_char().map(|s| s.chars().next()).flatten()?;
        if pred(chr) {
            self.next_char()
        } else {
            None
        }
    }

    fn next_while(&mut self, pred: impl FnMut(char) -> bool) -> &str {
        let start = self.chars.peek().map_or(self.text.len(), |&(pos, _)| pos);
        let size = 0;
        while let Some(len) = self.next_if(pred).map(|s| s.len()) {
            size += len;
        }

        self.text.get(start..start+size).unwrap()
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        let line = self.line;
        let col = self.col;
        
        let chr = self.cur_char().map(|s| s.chars().next()).flatten()?;
        
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        self.next_token()
    }
}
