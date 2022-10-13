use super::token::{Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnexpectedEof { pos: usize },
    InvalidCharacter { pos: usize, character: char },
    UnclosedTerminal { pos: usize },
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0 }
    }

    pub fn next(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        let peek = self
            .peek_chr()
            .ok_or(LexerError::UnexpectedEof { pos: self.pos })?;
        
        let token = match peek {
            ':' => self.take_char(TokenKind::Colon),
            ';' => self.take_char(TokenKind::Semicolon),
            '|' => self.take_char(TokenKind::Pipe),
            '(' => self.take_char(TokenKind::OpenParen),
            ')' => self.take_char(TokenKind::CloseParen),
            '[' => self.take_char(TokenKind::OpenBracket),
            ']' => self.take_char(TokenKind::CloseBracket),
            '{' => self.take_char(TokenKind::OpenBrace),
            '}' => self.take_char(TokenKind::CloseBrace),
            '\'' => {
                let start = self.pos;
                self.next_chr().unwrap(); // skip opening quote
                self.take_while(|c| c != '\'');
                self.next_chr().ok_or(LexerError::UnclosedTerminal { pos: start })?;
                Token {
                    kind: TokenKind::Terminal,
                    text: &self.input[start..self.pos],
                    pos: start,
                }
            }
            c if c.is_alphanumeric() || c == '_' => {
                let start = self.pos;
                let text = self.take_while(|c| c.is_alphanumeric() || c == '_');
                Token {
                    kind: TokenKind::NonTerminal,
                    text,
                    pos: start,
                }
            }
            _ => return Err(
                LexerError::InvalidCharacter { pos: self.pos, character: peek }
            ),
        };

        Ok(token)
    }

    fn take_char(&mut self, kind: TokenKind) -> Token {
        let start = self.pos;
        self.next_chr().unwrap();
        Token {
            kind,
            text: &self.input[start..self.pos],
            pos: start,
        }
    }

    fn peek_chr(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn next_chr(&mut self) -> Option<char> {
        let chr = self.peek_chr()?;
        self.pos += chr.len_utf8();
        Some(chr)
    }

    fn skip_while(&mut self, mut pred: impl FnMut(char) -> bool) {
        while let Some(chr) = self.peek_chr() {
            if !pred(chr) {
                break;
            }
            self.next_chr();
        }
    }

    fn take_while(&mut self, pred: impl FnMut(char) -> bool) -> &'a str {
        let start = self.pos;
        self.skip_while(pred);
        &self.input[start..self.pos]
    }

    fn skip_whitespace(&mut self) {
        loop {
            self.skip_while(char::is_whitespace);
            if self.peek_chr() == Some('#') {
                self.skip_while(|c| c != '\n');
            } else {
                break;
            }
        }
    }
}
