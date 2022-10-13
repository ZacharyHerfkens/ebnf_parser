#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
    NonTerminal,
    Terminal,
    Colon,
    Semicolon,
    Pipe,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub pos: usize,
}