use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenType {
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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::Identifier => write!(f, "<identifier>"),
            TokenType::String => write!(f, "\"<string literal>\""),
            TokenType::Number => write!(f, "<numeric literal>"),
            TokenType::And => write!(f, "and"),
            TokenType::Class => write!(f, "class"),
            TokenType::Else => write!(f, "else"),
            TokenType::False => write!(f, "false"),
            TokenType::For => write!(f, "for"),
            TokenType::Fun => write!(f, "fun"),
            TokenType::If => write!(f, "if"),
            TokenType::Nil => write!(f, "nil"),
            TokenType::Or => write!(f, "or"),
            TokenType::Print => write!(f, "print"),
            TokenType::Return => write!(f, "return"),
            TokenType::Super => write!(f, "super"),
            TokenType::This => write!(f, "this"),
            TokenType::True => write!(f, "true"),
            TokenType::Var => write!(f, "var"),
            TokenType::While => write!(f, "while"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct CodePosition {
    pub pos: usize,
    pub line: usize,
    pub column: usize,
}

impl Display for CodePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
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
    pub typ: TokenType,
    pub start: CodePosition,
    pub length: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} at {}", self.typ, self.start)
    }
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
    pub src: &'s str,
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
        while let Some(c) = self.peek().filter(|c| predicate(c)) {
            self.cursor.inc_for(c);
        }
        self.src[start.pos..self.cursor.pos].to_string()
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
        F: Fn(String) -> TokenType,
    {
        let start = self.cursor;
        let lexeme = self.eat(predicate);
        Token {
            start,
            length: lexeme.len(),
            typ: to_value(lexeme),
        }
    }

    fn pluck_token(&mut self, lexeme: char, value: TokenType) -> Token {
        let start = self.cursor;
        self.cursor.inc_for(lexeme);
        Token {
            start,
            length: lexeme.len_utf8(),
            typ: value,
        }
    }

    fn pluck_token_mod(
        &mut self,
        lexeme: char,
        modifier: char,
        value: TokenType,
        modified_value: TokenType,
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
            typ: if modified { modified_value } else { value },
        }
    }

    fn scan_number(&mut self) -> Token {
        let mut found_dot = false;
        self.scan_token(
            |c| {
                let is_dot = *c == '.';
                let res = c.is_ascii_digit() || (is_dot && !found_dot);
                if is_dot {
                    found_dot = true;
                }
                res
            },
            |_| TokenType::Number,
        )
    }

    fn scan_str(&mut self) -> ScanResult<Token> {
        self.cursor.inc_for('"');

        let token = self.scan_token(|c| c != &'"', |_| TokenType::String);

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
                "and" => TokenType::And,
                "class" => TokenType::Class,
                "else" => TokenType::Else,
                "false" => TokenType::False,
                "for" => TokenType::For,
                "fun" => TokenType::Fun,
                "if" => TokenType::If,
                "nil" => TokenType::Nil,
                "or" => TokenType::Or,
                "print" => TokenType::Print,
                "return" => TokenType::Return,
                "super" => TokenType::Super,
                "this" => TokenType::This,
                "true" => TokenType::True,
                "var" => TokenType::Var,
                "while" => TokenType::While,
                _ => TokenType::Identifier,
            },
        )
    }

    #[rustfmt::skip]
    fn scan(&mut self) -> Option<ScanResult<Token>> {
        self.eat_whitespace();
        let c = self.peek()?;
        match c {
            '(' => Some(Ok(self.pluck_token(c, TokenType::LeftParen))),
            ')' => Some(Ok(self.pluck_token(c, TokenType::RightParen))),
            '{' => Some(Ok(self.pluck_token(c, TokenType::LeftBrace))),
            '}' => Some(Ok(self.pluck_token(c, TokenType::RightBrace))),
            ';' => Some(Ok(self.pluck_token(c, TokenType::Semicolon))),
            ',' => Some(Ok(self.pluck_token(c, TokenType::Comma))),
            '.' => Some(Ok(self.pluck_token(c, TokenType::Dot))),
            '-' => Some(Ok(self.pluck_token(c, TokenType::Minus))),
            '+' => Some(Ok(self.pluck_token(c, TokenType::Plus))),
            '*' => Some(Ok(self.pluck_token(c, TokenType::Star))),
            '>' => Some(Ok(self.pluck_token_mod(c, '=', TokenType::Greater, TokenType::GreaterEqual))),
            '<' => Some(Ok(self.pluck_token_mod(c, '=', TokenType::Less, TokenType::LessEqual))),
            '!' => Some(Ok(self.pluck_token_mod(c, '=', TokenType::Bang, TokenType::BangEqual))),
            '=' => Some(Ok(self.pluck_token_mod(c, '=', TokenType::Equal, TokenType::EqualEqual))),
            '/' => {
                if self.peek_next() == Some('/') {
                    self.eat_to_eol();
                    self.scan()
                } else {
                    Some(Ok(self.pluck_token(c, TokenType::Slash)))
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
