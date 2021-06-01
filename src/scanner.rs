use std::fmt::Display;

use crate::fun::CopyExtensions;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenValue {
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

    // Width = 2: _[=]
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

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

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::LeftParen => write!(f, "("),
            TokenValue::RightParen => write!(f, ")"),
            TokenValue::LeftBrace => write!(f, "{{"),
            TokenValue::RightBrace => write!(f, "}}"),
            TokenValue::Comma => write!(f, ","),
            TokenValue::Dot => write!(f, "."),
            TokenValue::Minus => write!(f, "-"),
            TokenValue::Plus => write!(f, "+"),
            TokenValue::Semicolon => write!(f, ";"),
            TokenValue::Slash => write!(f, "/"),
            TokenValue::Star => write!(f, "*"),
            TokenValue::Bang => write!(f, "!"),
            TokenValue::BangEqual => write!(f, "!="),
            TokenValue::Equal => write!(f, "="),
            TokenValue::EqualEqual => write!(f, "=="),
            TokenValue::Greater => write!(f, ">"),
            TokenValue::GreaterEqual => write!(f, ">="),
            TokenValue::Less => write!(f, "<"),
            TokenValue::LessEqual => write!(f, "<="),
            TokenValue::Identifier => write!(f, "<identifier>"),
            TokenValue::String => write!(f, "\"<string literal>\""),
            TokenValue::Number => write!(f, "<numeric literal>"),
            TokenValue::And => write!(f, "and"),
            TokenValue::Class => write!(f, "class"),
            TokenValue::Else => write!(f, "else"),
            TokenValue::False => write!(f, "false"),
            TokenValue::For => write!(f, "for"),
            TokenValue::Fun => write!(f, "fun"),
            TokenValue::If => write!(f, "if"),
            TokenValue::Nil => write!(f, "nil"),
            TokenValue::Or => write!(f, "or"),
            TokenValue::Print => write!(f, "print"),
            TokenValue::Return => write!(f, "return"),
            TokenValue::Super => write!(f, "super"),
            TokenValue::This => write!(f, "this"),
            TokenValue::True => write!(f, "true"),
            TokenValue::Var => write!(f, "var"),
            TokenValue::While => write!(f, "while"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct CodePosition {
    pub pos: usize,
    pub line: usize,
    pub column: usize,
}

impl CodePosition {
    fn inc_for(&mut self, c: char) {
        let len = c.len_utf8();
        self.pos += c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            // This is an approximation because unicode is hard
            self.column += len;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Token {
    pub value: TokenValue,
    pub start: CodePosition,
    pub length: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScanError {
    pub value: ScanErrorValue,
    pub pos: CodePosition,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScanErrorValue {
    UnterminatedString(Token),
    UnexpectedCharacter(char),
}

pub type ScanResult<A> = Result<A, ScanError>;

pub struct Scanner<'s> {
    src: &'s str,
    cursor: CodePosition,
}
impl<'s> Scanner<'s> {
    pub fn new(src: &'s str) -> Scanner<'s> {
        Scanner {
            src,
            cursor: Default::default(),
        }
    }

    pub fn substr(&self, pos: CodePosition, length: usize) -> String {
        self.src[pos.pos..pos.pos + length].to_string()
    }

    fn char_at(&self, index: usize) -> Option<char> {
        self.src[index..].chars().next()
    }

    fn peek(&self) -> Option<char> {
        self.char_at(self.cursor.pos)
    }

    fn peek_next(&self) -> Option<char> {
        self.char_at(self.cursor.pos + 1)
    }

    fn eat<P>(&mut self, mut predicate: P) -> String
    where
        P: FnMut(&char) -> bool,
    {
        let start = self.cursor;
        loop {
            match self.peek().filter(|c| predicate(c)) {
                Some(c) => self.cursor.inc_for(c),
                None => {
                    break;
                }
            };
        }
        return self.src[start.pos..self.cursor.pos].to_string();
    }

    fn eat_whitespace(&mut self) {
        self.eat(|c| c.is_whitespace());
    }

    fn eat_to_eol(&mut self) {
        self.eat(|c| *c != '\n');
    }

    fn scan_token<P, F>(&mut self, predicate: P, to_value: F) -> Token
    where
        P: FnMut(&char) -> bool,
        F: Fn(String) -> TokenValue,
    {
        let start = self.cursor;
        let lexeme = self.eat(predicate);
        Token {
            start,
            length: lexeme.len(),
            value: to_value(lexeme),
        }
    }

    fn pluck_token(&mut self, lexeme: char, value: TokenValue) -> Token {
        let start = self.cursor;
        self.cursor.inc_for(lexeme);
        Token {
            start,
            length: lexeme.len_utf8(),
            value: value,
        }
    }

    fn pluck_token_mod(
        &mut self,
        lexeme: char,
        modifier: char,
        value: TokenValue,
        modified_value: TokenValue,
    ) -> Token {
        debug_assert_eq!(self.peek(), Some(lexeme));
        let start = self.cursor;
        self.cursor.inc_for(lexeme);

        let modified = self.peek() == Some(modifier);
        if modified {
            self.cursor.inc_for(modifier);
        }

        Token {
            start,
            length: if modified {
                lexeme.len_utf8() + modifier.len_utf8()
            } else {
                lexeme.len_utf8()
            },
            value: if modified { modified_value } else { value },
        }
    }

    fn scan_number(&mut self) -> Token {
        let mut found_dot = false;
        self.scan_token(
            |c| {
                let had_found_dot = found_dot.post_mut(|c| *c = true);
                c.is_ascii_digit() || (*c == '.' && !had_found_dot)
            },
            |_| TokenValue::Number,
        )
    }

    fn scan_str(&mut self) -> ScanResult<Token> {
        self.cursor.inc_for('"');

        let token = self.scan_token(|c| c != &'"', |_| TokenValue::String);

        if self.peek() == Some('"') {
            self.cursor.inc_for('"');
            Ok(token)
        } else {
            Err(ScanError {
                pos: token.start,
                value: ScanErrorValue::UnterminatedString(token),
            })
        }
    }

    fn scan_identifier(&mut self) -> Token {
        self.scan_token(
            |c| c.is_alphanumeric(),
            |l| match l.as_str() {
                "and" => TokenValue::And,
                "class" => TokenValue::Class,
                "else" => TokenValue::Else,
                "false" => TokenValue::False,
                "for" => TokenValue::For,
                "fun" => TokenValue::Fun,
                "if" => TokenValue::If,
                "nil" => TokenValue::Nil,
                "or" => TokenValue::Or,
                "print" => TokenValue::Print,
                "return" => TokenValue::Return,
                "super" => TokenValue::Super,
                "this" => TokenValue::This,
                "true" => TokenValue::True,
                "var" => TokenValue::Var,
                "while" => TokenValue::While,
                _ => TokenValue::Identifier,
            },
        )
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn scan(&mut self) -> Option<ScanResult<Token>> {
        self.eat_whitespace();
        let c = self.peek()?;
        match c {
            '(' => Some(Ok(self.pluck_token(c, TokenValue::LeftParen))),
            ')' => Some(Ok(self.pluck_token(c, TokenValue::RightParen))),
            '{' => Some(Ok(self.pluck_token(c, TokenValue::LeftBrace))),
            '}' => Some(Ok(self.pluck_token(c, TokenValue::RightBrace))),
            ';' => Some(Ok(self.pluck_token(c, TokenValue::Semicolon))),
            ',' => Some(Ok(self.pluck_token(c, TokenValue::Comma))),
            '.' => Some(Ok(self.pluck_token(c, TokenValue::Dot))),
            '-' => Some(Ok(self.pluck_token(c, TokenValue::Minus))),
            '+' => Some(Ok(self.pluck_token(c, TokenValue::Plus))),
            '*' => Some(Ok(self.pluck_token(c, TokenValue::Star))),
            '>' => Some(Ok(self.pluck_token_mod(c, '=', TokenValue::Greater, TokenValue::GreaterEqual))),
            '<' => Some(Ok(self.pluck_token_mod(c, '=', TokenValue::Less, TokenValue::LessEqual))),
            '!' => Some(Ok(self.pluck_token_mod(c, '=', TokenValue::Bang, TokenValue::BangEqual))),
            '=' => Some(Ok(self.pluck_token_mod(c, '=', TokenValue::Equal, TokenValue::EqualEqual))),
            '/' => {
                if self.peek_next() == Some('/') {
                    self.eat_to_eol();
                    self.scan()
                } else {
                    Some(Ok(self.pluck_token(c, TokenValue::Slash)))
                }
            },
            '"' => Some(self.scan_str()),
            c => {
                if c.is_ascii_digit() {
                    Some(Ok(self.scan_number()))
                } else if c.is_alphabetic() {
                    Some(Ok(self.scan_identifier()))
                } else {
                    Some(Err(ScanError {
                        pos: self.cursor,
                        value: ScanErrorValue::UnexpectedCharacter(c)
                    }))
                }
            }
        }
    }
}

impl<'s> Iterator for Scanner<'s> {
    type Item = ScanResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.scan()
    }
}
