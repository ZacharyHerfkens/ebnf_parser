#![cfg(test)]

use super::token::{Token, TokenKind};
use super::lexer::{Lexer, LexerError};

#[test]
fn test_lexer() {
    let input = "#comment\na: b;";
    let mut lexer = Lexer::new(input);
    let tokens = [
        Token { kind: TokenKind::NonTerminal, text: "a", pos: 9 },
        Token { kind: TokenKind::Colon, text: ":", pos: 10 },
        Token { kind: TokenKind::NonTerminal, text: "b", pos: 12 },
        Token { kind: TokenKind::Semicolon, text: ";", pos: 13 },
    ];
    for token in tokens.iter() {
        assert_eq!(lexer.next().unwrap(), *token);
    }
}

#[test]
fn test_lexer2() {
    let input = "a:'b'[c];";
    let mut lexer = Lexer::new(input);
    let tokens = [
        Token { kind: TokenKind::NonTerminal, text: "a", pos: 0 },
        Token { kind: TokenKind::Colon, text: ":", pos: 1 },
        Token { kind: TokenKind::Terminal, text: "'b'", pos: 2 },
        Token { kind: TokenKind::OpenBracket, text: "[", pos: 5 },
        Token { kind: TokenKind::NonTerminal, text: "c", pos: 6 },
        Token { kind: TokenKind::CloseBracket, text: "]", pos: 7 },
        Token { kind: TokenKind::Semicolon, text: ";", pos: 8 },
    ];
    for token in tokens.iter() {
        assert_eq!(lexer.next().unwrap(), *token);
    }
}

#[test]
fn test_lexer_invalid() {
    let input = "a%";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next(), Ok(Token {
        kind: TokenKind::NonTerminal,
        text: "a",
        pos: 0,
    }));
    assert_eq!(lexer.next(), Err(super::lexer::LexerError::InvalidCharacter {
        pos: 1,
        character: '%',
    }));
}

#[test]
fn test_lexer_unclosed() {
    let input = "'a";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next(), Err(LexerError::UnclosedTerminal { pos: 0 }));
}

fn test_lexer_eof() {
    let input = "";
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next(), Err(LexerError::UnexpectedEof { pos: 0 }));
}