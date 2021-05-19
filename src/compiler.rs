use std::{u32, usize};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }
}

enum Token1 {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
}

/// Two-character tokens for which
// the first character is a valid single-character token
enum Token2 {
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

enum Literal {
    Identifier,
    String,
    Number,
}

enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

struct CodePosition<'a> {
    data: &'a str,
    start: usize,
    line: u32,
}

struct TokenError<'a> {
    pos: CodePosition<'a>,
}

enum TokenType {
    Token1(Token1),
    Token2(Token2),
    Literal(Literal),
    Keyword(Keyword),
}

struct Token<'a> {
    typ: TokenType,
    pos: CodePosition<'a>,
}

pub struct Scanner<'a> {
    src: &'a str,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &str) -> Scanner {
        Scanner { src, line: 0 }
    }

    //fn scan_token(&self) -> Token {}

    fn scan(&self) {
        let mut line = 0_u32;
        // loop {
        // }
    }
}
