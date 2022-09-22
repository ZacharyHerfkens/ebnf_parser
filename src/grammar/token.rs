#[derive(Clone, Debug)]
pub enum TokenKind {
    Ident(String),
    Term(String),
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Pipe,
    Colon,
    SemiColon,
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub pos: usize
}