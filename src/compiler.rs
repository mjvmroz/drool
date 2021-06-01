use core::panic;
use std::mem;

use crate::{
    chunk::Chunk,
    op::Op,
    scanner::{CodePosition, ScanError, Scanner, Token, TokenType},
    value::Value,
};

#[derive(Clone, Debug, PartialEq)]
pub enum SyntaxError {
    UnexpectedEOF,
    ExpectedPrefix,
    ExpectedInfix,
    UnexpectedToken {
        pos: CodePosition,
        expected: TokenType,
        actual: TokenType,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompileError {
    Scan(ScanError),
    Syntax(SyntaxError),
    Internal(String),
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

pub struct Compiler<'s> {
    // TODO: look into peekable
    scanner: Scanner<'s>,
    previous: Option<Token>,
    current: Option<Token>,
    chunk: Chunk,
}

impl<'s> Compiler<'s> {
    fn new(src: &'s str) -> Compiler<'s> {
        Compiler {
            chunk: Chunk::default(),
            scanner: Scanner::new(src),
            previous: None,
            current: None,
        }
    }

    pub fn compile(src: &'s str) -> CompileResult<Chunk> {
        let mut compiler = Compiler::new(src);

        compiler.advance()?;
        compiler.expression()?;
        compiler
            .chunk
            .operation(Op::Return, compiler.get_previous()?.start.line);
        if cfg!(debug_assertions) {
            compiler.chunk.disassemble("Pre-exec disassembly");
            println!();
        }
        Ok(compiler.chunk)
    }

    fn current_precedence(&self) -> Precedence {
        self.current
            .map_or(Precedence::None, |c| Compiler::get_rule(c.typ).precedence)
    }

    fn get_previous(&self) -> CompileResult<Token> {
        self.previous
            .ok_or_else(|| SyntaxError::UnexpectedEOF.into())
    }

    fn advance(&mut self) -> CompileResult<()> {
        let _ = mem::replace(
            &mut self.previous,
            mem::replace(&mut self.current, self.scanner.next_token()?),
        );
        Ok(())
    }

    fn consume(&mut self, expected: TokenType) -> CompileResult<()> {
        let cur = self
            .current
            .ok_or(CompileError::Syntax(SyntaxError::UnexpectedEOF))?;
        if cur.typ == expected {
            self.advance()
        } else {
            Err(SyntaxError::UnexpectedToken {
                actual: cur.typ,
                expected,
                pos: cur.start,
            }
            .into())
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn execute(&mut self, instruction: ParseInstruction) -> CompileResult<()> {
        match instruction {
            ParseInstruction::Unary    => self.unary(),
            ParseInstruction::Binary   => self.binary(),
            ParseInstruction::Grouping => self.grouping(),
            ParseInstruction::Number   => self.number(),
        }
    }

    fn parse_precedence(&mut self, min: Precedence) -> CompileResult<()> {
        self.advance()?;

        let prefix_instruction = Compiler::get_rule(self.get_previous()?.typ)
            .prefix
            .ok_or(CompileError::Syntax(SyntaxError::ExpectedPrefix))?;

        self.execute(prefix_instruction)?;

        while min <= self.current_precedence() {
            self.advance()?;
            let infix_instruction = Compiler::get_rule(self.get_previous()?.typ)
                .infix
                .ok_or(CompileError::Syntax(SyntaxError::ExpectedInfix))?;
            self.execute(infix_instruction)?;
        }
        Ok(())
    }

    fn expression(&mut self) -> CompileResult<()> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn number(&mut self) -> CompileResult<()> {
        let prev = self.get_previous()?;
        let prev_type = prev.typ;
        match prev_type {
            TokenType::Number => {
                let s = self.scanner.substr(prev.start, prev.length);
                let val = s.parse().map_err(|err| {
                    CompileError::Internal(format!("Failed to parse number. Cause: {}", err))
                })?;
                self.chunk.push_const(Value::Double(val), prev.start.line);
                Ok(())
            }
            _ => Err(CompileError::Internal(format!(
                "Found '{}', expected number",
                prev_type
            ))),
        }
    }

    fn grouping(&mut self) -> CompileResult<()> {
        self.expression()?;
        self.consume(TokenType::RightParen)
    }

    fn unary(&mut self) -> CompileResult<()> {
        let token = self.get_previous()?;
        self.parse_precedence(Precedence::Unary)?;

        match token.typ {
            TokenType::Minus => Ok(self.chunk.operation(Op::Negate, token.start.line)),
            _ => Err(CompileError::Internal(format!(
                "Unhandled unary operator '{}'",
                token.typ
            ))),
        }
    }

    fn binary(&mut self) -> CompileResult<()> {
        let token = self.get_previous()?;

        let rule = Compiler::get_rule(token.typ);
        self.parse_precedence(rule.precedence.inc())?;

        match token.typ {
            TokenType::Plus => Ok(self.chunk.operation(Op::Add, token.start.line)),
            TokenType::Minus => Ok(self.chunk.operation(Op::Subtract, token.start.line)),
            TokenType::Star => Ok(self.chunk.operation(Op::Multiply, token.start.line)),
            TokenType::Slash => Ok(self.chunk.operation(Op::Divide, token.start.line)),
            _ => Err(CompileError::Internal(format!(
                "Unhandled binary operator: {}",
                token
            ))),
        }
    }


    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn get_rule(value: TokenType) -> ParseRule {
        match value {
            TokenType::LeftParen =>    ParseRule { prefix: Some(ParseInstruction::Grouping), infix: None,                           precedence: Precedence::None, },
            TokenType::RightParen =>   ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::LeftBrace =>    ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::RightBrace =>   ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Comma =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Dot =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Minus =>        ParseRule { prefix: Some(ParseInstruction::Unary),    infix: Some(ParseInstruction::Binary), precedence: Precedence::Term, },
            TokenType::Plus =>         ParseRule { prefix: None,                             infix: Some(ParseInstruction::Binary), precedence: Precedence::Term, },
            TokenType::Semicolon =>    ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Slash =>        ParseRule { prefix: None,                             infix: Some(ParseInstruction::Binary), precedence: Precedence::Factor, },
            TokenType::Star =>         ParseRule { prefix: None,                             infix: Some(ParseInstruction::Binary), precedence: Precedence::Factor, },
            TokenType::Bang =>         ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::BangEqual =>    ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Equal =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::EqualEqual =>   ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Greater =>      ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::GreaterEqual => ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Less =>         ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::LessEqual =>    ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Identifier =>   ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::String =>       ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Number =>       ParseRule { prefix: Some(ParseInstruction::Number),   infix: None,                           precedence: Precedence::None, },
            TokenType::And =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Class =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Else =>         ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::False =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::For =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Fun =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::If =>           ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Nil =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Or =>           ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Print =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Return =>       ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Super =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::This =>         ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::True =>         ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::Var =>          ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
            TokenType::While =>        ParseRule { prefix: None,                             infix: None,                           precedence: Precedence::None, },
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
            Precedence::Primary => panic!("it don't get higher than this"),
        }
    }
}
