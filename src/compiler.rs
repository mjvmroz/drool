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
    Unstructured {
        pos: CodePosition,
        message: String,
    },
    UnexpectedEOF,
    UnexpectedToken {
        pos: CodePosition,
        expected: TokenValue,
        description: String,
    },
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
    previous: Token,
    current: Token,
    chunk: Chunk,
}

impl<'s> Parser<'s> {
    pub fn parse(src: &'s str) -> CompileResult<Chunk> {
        let chunk = Chunk::default();
        let mut scanner = Scanner::new(src);
        let previous = scanner.next_token_or_err()?;
        let current = scanner.next_token_or_err()?;
        let mut parser = Parser {
            chunk,
            scanner,
            previous,
            current,
        };
        parser.expression()?;
        parser
            .chunk
            .operation(Op::Return, parser.current.start.line);
        Ok(parser.chunk)
    }

    fn advance(&mut self) -> CompileResult<()> {
        let previous = mem::replace(&mut self.current, self.scanner.next_token_or_err()?);
        self.previous = previous;
        Ok(())
    }

    fn consume(&mut self, expected: TokenValue) -> CompileResult<()> {
        if self.current.value == expected {
            self.advance()
        } else {
            Err(SyntaxError::UnexpectedToken {
                description: "ruh roh".to_string(),
                expected,
                pos: self.current.start,
            }
            .into())
        }
    }

    fn parsePrecedence(&mut self, precedence: Precedence) -> CompileResult<()> {
        todo!("uwu")
    }

    fn expression(&mut self) -> CompileResult<()> {
        self.parsePrecedence(Precedence::Assignment)?;
        Ok(())
    }

    fn number(&mut self) -> CompileResult<()> {
        match self.previous.value {
            TokenValue::Number(n) => {
                self.chunk
                    .push_const(Value::Double(n), self.previous.start.line);
                Ok(())
            }
            _ => panic!("TODO: this should be a result error"),
        }
    }

    fn grouping(&mut self) -> CompileResult<()> {
        self.expression()?;
        self.consume(TokenValue::RightParen)
    }

    fn unary(&mut self) -> CompileResult<()> {
        let (op, line) = self.previous.as_unary_op().expect("todo uwu");
        self.parsePrecedence(Precedence::Unary)?;

        Ok(self.chunk.operation(op, line))
    }

    fn binary(&mut self) -> CompileResult<()> {
        let (op, line) = self.previous.as_binary_op().expect("todo uwu");
        todo!("current")
    }
}

trait ScannerExtensions {
    fn next_token(&mut self) -> CompileResult<Option<Token>>;

    fn next_token_or_err(&mut self) -> CompileResult<Token> {
        let maybe_token = self.next_token()?;
        maybe_token.ok_or_else(|| CompileError::Syntax(SyntaxError::UnexpectedEOF))
    }
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
