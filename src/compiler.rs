use std::{convert::TryInto, intrinsics::transmute, iter, u32, usize};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    fn compile(src: &str) {
        loop {
            //let token = scanner;
            todo!("TODOUWU")
        }
    }
}

/// A coarse categorisation of tokens. Not a final representation,
/// but useful for a first parse. Each category has one byte of
/// contiguous space for easy expansion in future.
#[cfg_attr(rustfmt, rustfmt_skip)]
#[repr(u16)]
#[derive(Clone, Copy)]
enum TokenType {
    // Width = 1
    LeftParen     = 0x00_00,
    RightParen    = 0x00_01,
    LeftBrace     = 0x00_02,
    RightBrace    = 0x00_03,
    Comma         = 0x00_04,
    Dot           = 0x00_05,
    Minus         = 0x00_06,
    Plus          = 0x00_07,
    Semicolon     = 0x00_08,
    Slash         = 0x00_09,
    Star          = 0x00_0A,


    // Width = 1|2
    // Asserted property:
    // An '=' suffix means typ += 1
    Bang          = 0x01_00,
    BangEqual     = 0x01_01,

    Equal         = 0x01_02,
    EqualEqual    = 0x01_03,

    Greater       = 0x01_04,
    GreaterEqual  = 0x01_05,

    Less          = 0x01_06,
    LessEqual     = 0x01_07,


    // Literals
    Identifier    = 0x02_00,
    String        = 0x02_01,
    Number        = 0x02_02,


    // Keywords
    And           = 0x03_01,
    Class         = 0x03_02,
    Else          = 0x03_03,
    False         = 0x03_04,
    For           = 0x03_05,
    Fun           = 0x03_06,
    If            = 0x03_07,
    Nil           = 0x03_08,
    Or            = 0x03_09,
    Print         = 0x03_0A,
    Return        = 0x03_0B,
    Super         = 0x03_0C,
    This          = 0x03_0D,
    True          = 0x03_0E,
    Var           = 0x03_0F,
    While         = 0x03_10,
}

enum ConsumeOutcome {
    Consumed { flush: bool },
    NotConsumed,
}

impl Token {
    fn try_consume(&mut self, c: char) -> bool {
        unsafe {
            let code: u16 = transmute(self.typ);
            // Is partial { width = 1|2 } operator and next char extends it:
            if code & 0xFF_01 == 0x01_00 && c == '=' {
                self.typ = transmute(code + 1);
                self.pos.length += 1;
                return true;
            }
            false
        }
    }
}

struct CodePosition {
    start: usize,
    length: usize,
    line: u32,
}

struct TokenError {
    pos: CodePosition,
}

pub struct Token {
    typ: TokenType,
    pos: CodePosition,
}

#[derive(Default)]
struct ScannerState {
    pub line: u32,
    pub start: u32,
    pub pending_token: Option<Token>,
}

struct Scanner {}
impl Scanner {
    fn scan_char(state: &mut ScannerState, pos: usize, current_char: char) -> Vec<Token> {
        //fn tokens(typ: TokenType) -> Vec<Token> {}
        state
            .pending_token
            .as_mut()
            .map(|t| t.try_consume(current_char));
        let token: Option<TokenType> = match current_char {
            '\n' => {
                state.line += 1;
                None
            }
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ';' => Some(TokenType::Semicolon),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            '/' => Some(TokenType::Slash),
            '*' => Some(TokenType::Star),

            _ => todo!("nani"),
        };
        // match (state.last_char, current_char) {
        //     (Some(last), cur) => todo!(),
        //     (None, cur) => todo!(),
        // }
        //state.last_char = Some(current_char);
        todo!()
    }

    fn scan<'a>(src: &'a str) {
        let res = src
            .char_indices()
            .scan(ScannerState::default(), |state, (pos, cur)| {
                Some(Scanner::scan_char(state, pos, cur))
            });

        let _tokens: Vec<Token> = res.flat_map(Vec::into_iter).collect();
    }
}
