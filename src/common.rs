use num_enum::IntoPrimitive;
use num_enum::UnsafeFromPrimitive;

#[derive(Debug, IntoPrimitive, UnsafeFromPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpNil,
    OpTrue,
    OpFalse,
    OpEqual,
    OpGreater,
    OpLess,
    OpGreaterEqual,
    OpLessEqual,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNot,
    OpNotEqual, // OpTest,
                // OpChoose
}

#[derive(Debug, IntoPrimitive, UnsafeFromPrimitive, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum TokenType {
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

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    MinusEqual,
    PlusEqual,
    SlashEqual,
    StarEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    StringToken,
    Number,

    // Keywords.
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

    Eof,

    Question,
    Colon,
}
