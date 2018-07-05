#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Int(i32),
    Str(String),
    None,
    True,
    False,
    If,
    Else,
    While,
    Break,
    Continue,
    Def,
    Return,
    Assert,
    Class,
    Plus,
    Eq,
    EqEq,
    Lt,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Colon,
    Comma,
    Dot,
    NewLine,
    Indent,
    Dedent,
    EOF,
}