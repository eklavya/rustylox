use crate::common::TokenType;
use crate::common::TokenType::Eof;
use crate::scanner::{Scanner, Token};
use num_enum::IntoPrimitive;
use num_enum::UnsafeFromPrimitive;

pub struct Parser<'source_lifetime> {
    pub current: Option<Token<'source_lifetime>>,
    pub prev: Option<Token<'source_lifetime>>,
    pub had_error: bool,
    pub panic_mode: bool,
    pub scanner: Scanner<'source_lifetime>,
}

#[derive(Debug, IntoPrimitive, UnsafeFromPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum Precedence {
    PrecNone,
    PrecAssignment, // =
    PrecTernary,
    PrecOr,         // or
    PrecAnd,        // and
    PrecEquality,   // == !=
    PrecComparison, // < > <= >=
    PrecTerm,       // + -
    PrecFactor,     // * /
    PrecUnary,      // ! -
    PrecCall,       // . ()
    PrecPrimary,
}

impl<'source_lifetime> Parser<'source_lifetime> {
    pub fn new(source: &'source_lifetime str) -> Self {
        Parser {
            current: None,
            prev: None,
            had_error: false,
            panic_mode: false,
            scanner: Scanner::new(source),
        }
    }

    pub fn advance(&mut self) {
        self.prev = std::mem::replace(&mut self.current, None);

        loop {
            let token = self.scanner.scan_token();
            // println!("token is {:?}", token);
            match token {
                Ok(t) => {
                    self.current = Some(t);
                    break;
                }
                Err(msg) => {
                    self.error_at_current(msg);
                    self.had_error = true;
                }
            }
        }
    }

    pub fn error_at_current(&mut self, msg: &'static str) {
        self.error_at("current", msg);
    }

    pub fn error(&mut self, msg: &'static str) {
        self.error_at("prev", msg);
    }

    fn error_at(&mut self, token: &str, msg: &'static str) {
        let t = if token.eq("current") {
            self.current.as_ref().unwrap()
        } else {
            self.prev.as_ref().unwrap()
        };
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}] Error", t.line);
        if t.token_type == Eof {
            eprint!(" at end");
        } else {
            eprint!(" at {}", t.lexeme);
        }

        eprintln!(": {}", msg);
        self.had_error = true;
    }

    pub fn consume(&mut self, token_type: TokenType, msg: &'static str) {
        if self.current.as_ref().unwrap().token_type == token_type {
            self.advance()
        } else {
            self.error_at_current(msg);
        }
    }
}
