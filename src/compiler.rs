use crate::chunk::Chunk;
use crate::common::OpCode::*;
use crate::common::TokenType;
use crate::common::TokenType::{Eof, RightParen};
use crate::parser::Precedence::*;
use crate::parser::{Parser, Precedence};
use crate::value::Value;

type ParseFn = Option<fn(&mut Compiler) -> ()>;

pub struct ParseRule {
    pub prefix: ParseFn,
    pub infix: ParseFn,
    pub precedence: Precedence,
}

pub struct Compiler<'source_lifetime> {
    parser: Parser<'source_lifetime>,
    chunk: &'source_lifetime mut Chunk,
}

impl<'source_lifetime> Compiler<'source_lifetime> {
    const PARSE_RULES: [ParseRule; 44] = [
        ParseRule {
            prefix: Some(Compiler::grouping),
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_LEFT_PAREN 0
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_RIGHT_PAREN 1
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_LEFT_BRACE 2
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_RIGHT_BRACE 3
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_COMMA 4
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_DOT 5
        ParseRule {
            prefix: Some(Compiler::unary),
            infix: Some(Compiler::binary),
            precedence: PrecTerm,
        }, // TOKEN_MINUS 6
        ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecTerm,
        }, // TOKEN_PLUS 7
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_SEMICOLON 8
        ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecFactor,
        }, // TOKEN_SLASH 9
        ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecFactor,
        }, // TOKEN_STAR 10
        ParseRule {
            prefix: Some(Compiler::unary),
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_BANG 11
        ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecEquality,
        }, // TOKEN_BANG_EQUAL 12
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_EQUAL 13
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_PLUS_EQUAL 14
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_MINUS_EQUAL 15
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_STAR_EQUAL 16
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_SLASH_EQUAL 17
        ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecComparison,
        }, // TOKEN_EQUAL_EQUAL 18
        ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecComparison,
        }, // TOKEN_GREATER 19
        ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecComparison,
        }, // TOKEN_GREATER_EQUAL 20
        ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecComparison,
        }, // TOKEN_LESS 21
        ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecComparison,
        }, // TOKEN_LESS_EQUAL 22
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_IDENTIFIER 23
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_STRING 24
        ParseRule {
            prefix: Some(Compiler::number),
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_NUMBER 25
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_AND 26
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_CLASS 27
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_ELSE 28
        ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_FALSE 29
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_FOR 30
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_FUN 31
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_IF 32
        ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_NIL 33
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_OR 34
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_PRINT 35
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_RETURN 36
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_SUPER 37
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_THIS 38
        ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_TRUE 39
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_VAR 40
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_WHILE 41
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_ERROR 42
        ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }, // TOKEN_EOF 43
           // ParseRule {
           //     prefix: None,
           //     infix: Some(Compiler::choose),
           //     precedence: PrecTernary
           // }, // TOKEN_QUESTION
           // ParseRule {
           //     prefix: None,
           //     infix: Some(Compiler::question),
           //     precedence: PrecTernary
           // }, // TOKEN_COLON
    ];

    const PARSE_RULES_DBG: [&'static str; 44] = [
        "ParseRule {
            prefix: Some(Compiler::grouping),
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_LEFT_PAREN
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_RIGHT_PAREN
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_LEFT_BRACE
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_RIGHT_BRACE
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_COMMA
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_DOT
        "ParseRule {
            prefix: Some(Compiler::unary),
            infix: Some(Compiler::binary),
            precedence: PrecTerm,
        }", // TOKEN_MINUS
        "ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecTerm,
        }", // TOKEN_PLUS
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_SEMICOLON
        "ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecFactor,
        }", // TOKEN_SLASH
        "ParseRule {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: PrecFactor,
        }", // TOKEN_STAR
        "ParseRule {
            prefix: Some(Compiler::unary),
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_BANG
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_BANG_EQUAL
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_EQUAL
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_PLUS_EQUAL
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_MINUS_EQUAL
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_STAR_EQUAL
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_SLASH_EQUAL
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_EQUAL_EQUAL
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_GREATER
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_GREATER_EQUAL
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_LESS
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_LESS_EQUAL
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_IDENTIFIER
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_STRING
        "ParseRule {
            prefix: Some(Compiler::number),
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_NUMBER
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_AND
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_CLASS
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_ELSE
        "ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_FALSE
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_FOR
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_FUN
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_IF
        "ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_NIL
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_OR
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_PRINT
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_RETURN
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_SUPER
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_THIS
        "ParseRule {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_TRUE
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_VAR
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_WHILE
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_ERROR
        "ParseRule {
            prefix: None,
            infix: None,
            precedence: PrecNone,
        }", // TOKEN_EOF
            // ParseRule {
            //     prefix: None,
            //     infix: Some(Compiler::choose),
            //     precedence: PrecTernary
            // }, // TOKEN_QUESTION
            // ParseRule {
            //     prefix: None,
            //     infix: Some(Compiler::question),
            //     precedence: PrecTernary
            // }, // TOKEN_COLON
    ];

    pub fn new(source: &'source_lifetime str, chunk: &'source_lifetime mut Chunk) -> Self {
        Compiler {
            parser: Parser::new(source),
            chunk,
        }
    }

    pub fn compile(&mut self, source: &'source_lifetime str) -> bool {
        self.parser = Parser::new(source);
        self.parser.advance();
        self.expression();
        self.parser.consume(Eof, "Expected end of expression.");
        self.end_compiler();
        return !self.parser.had_error;
    }

    fn emit_byte(&mut self, byte: u8) {
        self.chunk
            .write(byte, self.parser.prev.as_ref().unwrap().line);
    }

    fn end_compiler(&mut self) {
        self.emit_byte(OpReturn.into());
        if !self.parser.had_error {
            self.chunk.disassemble("code");
        }
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn grouping(comp: &mut Compiler) {
        comp.expression();
        comp.parser
            .consume(RightParen, "Expected ) after expression.");
    }

    // fn choose(comp: &mut Compiler) {
    //     comp.expression();
    //     comp.parse_precedence(PrecTernary);
    // }

    fn unary(comp: &mut Compiler) {
        let operator_type = comp.parser.prev.as_ref().unwrap().token_type;
        comp.parse_precedence(PrecUnary);
        match operator_type {
            TokenType::Minus => comp.emit_byte(OpNegate.into()),
            TokenType::Bang => comp.emit_byte(OpNot.into()),
            _ => {}
        }
    }

    fn binary(comp: &mut Compiler) {
        let operator_type = comp.parser.prev.as_ref().unwrap().token_type;
        let ind: u8 = operator_type.into();
        let rule: &ParseRule = &Compiler::PARSE_RULES[ind as usize];
        let prec: u8 = rule.precedence.into();
        comp.parse_precedence(unsafe { Precedence::from_unchecked(prec + 1) });
        match operator_type {
            TokenType::Plus => comp.emit_byte(OpAdd.into()),
            TokenType::Minus => comp.emit_byte(OpSubtract.into()),
            TokenType::Star => comp.emit_byte(OpMultiply.into()),
            TokenType::Slash => comp.emit_byte(OpDivide.into()),
            TokenType::BangEqual => comp.emit_byte(OpNotEqual.into()),
            TokenType::EqualEqual => comp.emit_byte(OpEqual.into()),
            TokenType::Greater => comp.emit_byte(OpGreater.into()),
            TokenType::GreaterEqual => comp.emit_byte(OpGreaterEqual.into()),
            TokenType::Less => comp.emit_byte(OpLess.into()),
            TokenType::LessEqual => comp.emit_byte(OpLessEqual.into()),
            _ => {}
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.parser.advance();
        // println!("parser is {:?} {:?}", self.parser.prev, self.parser.current);
        let ind: u8 = self.parser.prev.as_ref().unwrap().token_type.into();
        // println!("token type should be {:?} and is {:?}", self.parser.prev.as_ref().unwrap().token_type, ind);
        // println!(
        //     "precedence for {:?} is {}",
        //     precedence,
        //     Compiler::PARSE_RULES_DBG[ind as usize]
        // );
        let rule: &ParseRule = &Compiler::PARSE_RULES[ind as usize];
        if rule.prefix.is_none() {
            self.parser.error("Expected expression.");
        } else {
            rule.prefix.unwrap()(self);
            // println!("will check precedence for {:?}", precedence);
            while self.is_lower_prec(precedence) {
                self.parser.advance();
                // println!("parser is {:?} {:?}", self.parser.prev, self.parser.current);
                let ind: u8 = self.parser.prev.as_ref().unwrap().token_type.into();
                // println!(
                //     "precedence for {:?} is {}",
                //     precedence,
                //     Compiler::PARSE_RULES_DBG[ind as usize]
                // );
                let pr: &ParseRule = &Compiler::PARSE_RULES[ind as usize];
                pr.infix.unwrap()(self);
            }
        }
    }

    #[inline(always)]
    fn is_lower_prec(&self, prec: Precedence) -> bool {
        let p: u8 = self.parser.current.as_ref().unwrap().token_type.into();
        let pr: &ParseRule = &Compiler::PARSE_RULES[p as usize];
        let p1: u8 = prec.into();
        p1 <= pr.precedence.into()
    }

    fn expression(&mut self) {
        self.parse_precedence(PrecAssignment);
    }

    fn number(comp: &mut Compiler) {
        let value = comp
            .parser
            .prev
            .as_ref()
            .unwrap()
            .lexeme
            .parse::<f64>()
            .unwrap();
        comp.emit_constant(Value::Number(value));
    }

    fn literal(comp: &mut Compiler) {
        match comp.parser.prev.as_ref().unwrap().token_type {
            TokenType::False => comp.emit_byte(OpFalse.into()),
            TokenType::True => comp.emit_byte(OpTrue.into()),
            TokenType::Nil => comp.emit_byte(OpNil.into()),
            _ => return,
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let ind = self.make_constant(value);
        self.emit_bytes(OpConstant.into(), ind);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.chunk.add_constant(value);
        if constant > 255 {
            self.parser.error("Can not have more than 255 constants");
            0
        } else {
            constant.into()
        }
    }
}
