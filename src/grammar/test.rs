#![cfg(test)]

use super::{lexer::Lexer, token::TokenKind};

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new(r"( 'ab\'' Def ) ;");
    let expected = vec!["(", "'ab\\''", "Def", ")", ";"];
    for (i, token) in lexer.enumerate() {
        assert_eq!(token.text, expected[i]);
    }
}

#[test]
fn test_lexer2() {
    let mut lexer = Lexer::new(r"'ab\'\n'");
    let tok = lexer.next().unwrap();
    match tok.kind {
        TokenKind::Term(s) => assert_eq!(s, "ab'\n"),
        _ => panic!("Expected terminal"),
    }
}