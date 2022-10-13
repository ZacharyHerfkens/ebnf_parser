#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub pos: usize,
}