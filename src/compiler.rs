use core::panic;
use std::mem;

use crate::{
    chunk::Chunk,
    op::Op,
    scanner::{CodePosition, ScanError, Scanner, Token, TokenValue},
    value::Value,
};

#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxError {
    UnexpectedEOF,
    UnexpectedToken {
        pos: CodePosition,
        expected: TokenValue,
        actual: TokenValue,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompileError {
    Scan(ScanError),
    Syntax(SyntaxError),
}

impl From<SyntaxError> for CompileError {
    fn from(e: SyntaxError) -> Self {
        CompileError::Syntax(e)
    }
}

impl From<ScanError> for CompileError {
    fn from(e: ScanError) -> Self {
        CompileError::Scan(e)
    }
}

type CompileResult<A> = Result<A, CompileError>;

pub struct Compiler {}

impl Compiler {
    pub fn compile<'s>(src: &'s str) -> CompileResult<Chunk> {
        Parser::parse(src)
    }
}

pub struct Parser<'s> {
    // TODO: look into peekable
    scanner: Scanner<'s>,
    previous: Option<Token>,
    current: Option<Token>,
    chunk: Chunk,
}

impl<'s> Parser<'s> {
    fn current_precedence(&self) -> Precedence {
        self.current
            .map_or(Precedence::None, |c| Parser::get_rule(c.value).precedence)
    }

    fn get_previous(&self) -> Token {
        self.previous.expect("If this is None we've fucky wuckied.")
    }

    pub fn parse(src: &'s str) -> CompileResult<Chunk> {
        let mut parser = Parser {
            chunk: Chunk::default(),
            scanner: Scanner::new(src),
            previous: None,
            current: None,
        };

        parser.advance()?;
        parser.expression()?;
        parser
            .chunk
            .operation(Op::Return, parser.get_previous().start.line);
        if cfg!(debug_assertions) {
            parser.chunk.disassemble("Pre-exec disassembly");
            println!();
        }
        Ok(parser.chunk)
    }

    fn advance(&mut self) -> CompileResult<()> {
        let _ = mem::replace(
            &mut self.previous,
            mem::replace(&mut self.current, self.scanner.next_token()?),
        );
        Ok(())
    }

    fn consume(&mut self, expected: TokenValue) -> CompileResult<()> {
        let cur = self
            .current
            .ok_or(CompileError::Syntax(SyntaxError::UnexpectedEOF))?;
        if cur.value == expected {
            self.advance()
        } else {
            Err(SyntaxError::UnexpectedToken {
                actual: cur.value,
                expected,
                pos: cur.start,
            }
            .into())
        }
    }

    fn execute(&mut self, instruction: ParseInstruction) -> CompileResult<()> {
        match instruction {
            ParseInstruction::Unary => self.unary(),
            ParseInstruction::Binary => self.binary(),
            ParseInstruction::Grouping => self.grouping(),
            ParseInstruction::Number => self.number(),
        }
    }

    fn parse_precedence(&mut self, up_to: Precedence) -> CompileResult<()> {
        self.advance()?;

        let prefix_instruction = Parser::get_rule(self.get_previous().value)
            .prefix
            .expect("expected prefix expression");

        self.execute(prefix_instruction)?;

        while up_to <= self.current_precedence() {
            self.advance()?;
            let infix_instruction = Parser::get_rule(self.get_previous().value)
                .infix
                .expect("expected infix expression");
            self.execute(infix_instruction)?;
        }
        Ok(())
    }

    fn expression(&mut self) -> CompileResult<()> {
        self.parse_precedence(Precedence::Assignment)?;
        Ok(())
    }

    fn number(&mut self) -> CompileResult<()> {
        match self.get_previous().value {
            TokenValue::Number => {
                let s = self
                    .scanner
                    .substr(self.get_previous().start, self.get_previous().length);
                self.chunk.push_const(
                    Value::Double(s.parse().expect("TODO: this should be an error")),
                    self.get_previous().start.line,
                );
                Ok(())
            }
            _ => panic!("TODO: this should be an error"),
        }
    }

    fn grouping(&mut self) -> CompileResult<()> {
        self.expression()?;
        self.consume(TokenValue::RightParen)
    }

    fn unary(&mut self) -> CompileResult<()> {
        let token = self.get_previous();
        self.parse_precedence(Precedence::Unary)?;

        match token.value {
            TokenValue::Minus => Ok(self.chunk.operation(Op::Negate, token.start.line)),
            _ => todo!("unreachable - should be fatal compile error"),
        }
    }

    fn binary(&mut self) -> CompileResult<()> {
        let token = self.get_previous();

        let rule = Parser::get_rule(token.value);
        self.parse_precedence(rule.precedence.inc())?;

        match token.value {
            TokenValue::Plus => Ok(self.chunk.operation(Op::Add, token.start.line)),
            TokenValue::Minus => Ok(self.chunk.operation(Op::Subtract, token.start.line)),
            TokenValue::Star => Ok(self.chunk.operation(Op::Multiply, token.start.line)),
            TokenValue::Slash => Ok(self.chunk.operation(Op::Divide, token.start.line)),
            _ => todo!("unreachable - should be fatal compile error"),
        }
    }


    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn get_rule(value: TokenValue) -> ParseRule {
        match value {
            TokenValue::LeftParen =>    ParseRule { prefix: Some(ParseInstruction::Grouping), infix: None,                           precedence: Precedence::None, },
            TokenValue::RightParen =>   ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::LeftBrace =>    ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::RightBrace =>   ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Comma =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Dot =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Minus =>        ParseRule { prefix: Some(ParseInstruction::Unary),    infix: Some(ParseInstruction::Binary), precedence: Precedence::Term, },
            TokenValue::Plus =>         ParseRule { prefix: None,                             infix: Some(ParseInstruction::Binary), precedence: Precedence::Term, },
            TokenValue::Semicolon =>    ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Slash =>        ParseRule { prefix: None,                             infix: Some(ParseInstruction::Binary), precedence: Precedence::Factor, },
            TokenValue::Star =>         ParseRule { prefix: None,                             infix: Some(ParseInstruction::Binary), precedence: Precedence::Factor, },
            TokenValue::Bang =>         ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::BangEqual =>    ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Equal =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::EqualEqual =>   ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Greater =>      ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::GreaterEqual => ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Less =>         ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::LessEqual =>    ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Identifier =>   ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::String =>       ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Number =>       ParseRule { prefix: Some(ParseInstruction::Number),   infix: None,                           precedence: Precedence::None, },
            TokenValue::And =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Class =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Else =>         ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::False =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::For =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Fun =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::If =>           ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Nil =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Or =>           ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Print =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Return =>       ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Super =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::This =>         ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::True =>         ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::Var =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenValue::While =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
        }
    }
}

trait ScannerExtensions {
    fn next_token(&mut self) -> CompileResult<Option<Token>>;
}

impl<'s> ScannerExtensions for Scanner<'s> {
    fn next_token(&mut self) -> CompileResult<Option<Token>> {
        let maybe_next_or_err = self
            .map(|r| r.map_err::<CompileError, _>(|e| e.into()))
            .next();

        match maybe_next_or_err {
            Some(res) => res.map(Option::Some),
            None => Ok(None),
        }
    }
}

enum ParseInstruction {
    Unary,
    Binary,
    Grouping,
    Number,
}

struct ParseRule {
    prefix: Option<ParseInstruction>,
    infix: Option<ParseInstruction>,
    precedence: Precedence,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl Precedence {
    pub fn inc(&self) -> Precedence {
        match self {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => panic!("bonk that's a dummkopf play"),
        }
    }
}
