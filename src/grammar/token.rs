pub enum TokenKind {
    Ident,
    Term,
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

pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub line: usize,
    pub col: usize,
}