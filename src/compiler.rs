use core::f64;
use std::usize;

use crate::fun::CopyExtensions;
use crate::vm::{InterpretError, InterpretResult};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile(src: &str) {
        loop {
            //let token = scanner;
            //let token = Scanner::scanToken();
        }
    }
}

pub struct Interpreter {}
impl Interpreter {
    fn interpret(src: &str) -> InterpretResult<()> {
        Compiler::compile(src);
        Ok(())
    }
}

#[derive(PartialEq)]
enum TokenValue {
    // Width = 1
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

    // Width = 1|2
    // Asserted property:
    // An '=' suffix means typ += 1
    Bang,
    BangEqual,

    Equal,
    EqualEqual,

    Greater,
    GreaterEqual,

    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
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

#[derive(Default, Clone, Copy)]
struct CodePosition {
    pos: usize,
    line: usize,
    column: usize,
}

impl CodePosition {
    // Returns a constant true for convenient use with take_while
    fn inc(&mut self) -> bool {
        self.inc_by(1);
        true
    }

    fn inc_by(&mut self, i: usize) {
        self.pos += i;
        self.column += i;
    }

    // Returns a constant true for convenient use with take_while
    fn inc_ln(&mut self) -> bool {
        self.pos += 1;
        self.line += 1;
        self.column = 0;
        true
    }
}

pub struct Token {
    value: TokenValue,
    start: CodePosition,
    length: usize,
}

pub struct ScanError {
    value: ScanErrorValue,
    pos: CodePosition,
}

pub enum ScanErrorValue {
    UnterminatedString,
}

pub type ScanResult<A> = Result<A, ScanError>;

#[derive(Default)]
struct Scanner<'s> {
    src: &'s str,
    cursor: CodePosition,
}
impl<'s> Scanner<'s> {
    fn new(src: &'s str) -> Scanner<'s> {
        Scanner {
            src,
            ..Default::default()
        }
    }

    fn char_at(&self, index: usize) -> Option<char> {
        self.src[index..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.char_at(self.cursor.pos + 1)
    }

    fn is_end(&self) -> bool {
        self.peek_next() == None
    }

    fn eat_whitespace(&mut self) {
        let _lexeme = self.src[self.cursor.pos..]
            .chars()
            .take_while(|c| match c {
                '\n' => self.cursor.inc_ln(),
                c => c.is_whitespace() && self.cursor.inc(),
            })
            .collect::<String>();
    }

    /// Designed for use with line comments.
    fn eat_to_eol(&mut self) {
        let lexeme = self.src[self.cursor.pos..]
            .chars()
            .take_while(|c| *c != '\n')
            .collect::<String>();

        // We assert that the consumed characters contain no line breaks.
        // This permits us to be lazy and use a single `inc_by` rather than the
        // messy affair which is case-by-case matching.
        self.cursor.inc_by(lexeme.len());
    }

    fn scan_number(&mut self) -> Token {
        // Keep track of where we've seen a dot. We're not fussy about
        // it starting or terminating numbers, but we don't want any
        // IP addresses floatin' in. We don't serve their kind here. 👿
        let mut found_dot = false;
        let lexeme = self.src[self.cursor.pos..]
            .chars()
            .take_while(|c| c.is_numeric() || *c == '.' && !found_dot.post_mut(|c| *c = true))
            .collect::<String>();

        // See note on assertion in {skip_comment}
        debug_assert!(!lexeme.contains('\n'));
        let token = Token {
            start: self.cursor,
            length: lexeme.len(),
            value: TokenValue::Number(lexeme.parse().unwrap()),
        };
        self.cursor.inc_by(lexeme.len());
        token
    }

    fn scan_str(&mut self) -> ScanResult<Token> {
        // Rust strings are UTF-8, and we're iterating by Unicode
        // code point, so it's bad practice to assume the width of
        // characters (even though in this case I'm 99.999%
        // confident it's one byte).
        let start = self.cursor;

        debug_assert_eq!(self.peek_next(), Some('"'));
        self.cursor.inc_by('"'.len_utf8());

        let lexeme = self.src[self.cursor.pos..]
            .chars()
            .take_while(|c| match c {
                '\n' => {
                    self.cursor.inc_ln();
                    true
                }
                c => {
                    if c != &'"' {
                        self.cursor.inc();
                        true
                    } else {
                        false
                    }
                }
            })
            .collect::<String>();

        if self.peek_next() == Some('"') {
            Ok(Token {
                start,
                length: lexeme.len(),
                value: TokenValue::String(lexeme),
            })
        } else {
            Err(ScanError {
                pos: start,
                value: ScanErrorValue::UnterminatedString,
            })
        }
    }

    // fn scan_token(&mut self) -> TokenValue {
    //     self.start = self.cursor.pos;
    // }

    // '(' => Some(TokenType::LeftParen),
    // ')' => Some(TokenType::RightParen),
    // '{' => Some(TokenType::LeftBrace),
    // '}' => Some(TokenType::RightBrace),
    // ';' => Some(TokenType::Semicolon),
    // ',' => Some(TokenType::Comma),
    // '.' => Some(TokenType::Dot),
    // '-' => Some(TokenType::Minus),
    // '+' => Some(TokenType::Plus),
    // '/' => Some(TokenType::Slash),
    // '*' => Some(TokenType::Star),
}
