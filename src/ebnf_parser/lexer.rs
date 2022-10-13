use super::token::{Token, TokenKind};

pub struct Lexer {
    input: String,
    pos: usize,
}

pub enum LexerError {
    UnexpectedEof { pos: usize },
    InvalidCharacter { pos: usize, character: char },
}
