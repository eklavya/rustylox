use crate::common::TokenType;
use crate::common::TokenType::*;

#[derive(Debug)]
pub struct Scanner<'source_lifetime> {
    start: usize,
    current: usize,
    source: &'source_lifetime str,
    line: i32,
}

#[derive(Debug)]
pub struct Token<'source_lifetime> {
    pub token_type: TokenType,
    pub lexeme: &'source_lifetime str,
    pub line: i32,
}

impl<'source_lifetime> Scanner<'source_lifetime> {
    pub fn new(source: &'source_lifetime str) -> Self {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            source,
        }
    }

    // pub fn init_scanner(&mut self, source: &'source_lifetime str) {
    //     self.start = 0;
    //     self.current = 0;
    //     self.line = 1;
    //     self.source = source;
    // }

    pub fn scan_token(&mut self) -> Result<Token<'source_lifetime>, &'static str> {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            Ok(self.make_token(Eof))
        } else {
            let c = self.advance();
            match c {
                '(' => Ok(self.make_token(LeftParen)),
                ')' => Ok(self.make_token(RightParen)),
                '{' => Ok(self.make_token(LeftBrace)),
                '}' => Ok(self.make_token(RightBrace)),
                ';' => Ok(self.make_token(Semicolon)),
                ',' => Ok(self.make_token(Comma)),
                '.' => Ok(self.make_token(Dot)),
                '-' => {
                    let token = self.get_token_type('=', MinusEqual, Minus);
                    Ok(self.make_token(token))
                }
                '+' => {
                    let token = self.get_token_type('=', PlusEqual, Plus);
                    Ok(self.make_token(token))
                }
                '/' => {
                    let token = self.get_token_type('=', SlashEqual, Slash);
                    Ok(self.make_token(token))
                }
                '*' => {
                    let token = self.get_token_type('=', StarEqual, Star);
                    Ok(self.make_token(token))
                }
                '!' => {
                    let token = self.get_token_type('=', BangEqual, Bang);
                    Ok(self.make_token(token))
                }
                '=' => {
                    let token = self.get_token_type('=', EqualEqual, Equal);
                    Ok(self.make_token(token))
                }
                '<' => {
                    let token = self.get_token_type('=', LessEqual, Less);
                    Ok(self.make_token(token))
                }
                '>' => {
                    let token = self.get_token_type('=', GreaterEqual, Greater);
                    Ok(self.make_token(token))
                }
                '?' => Ok(self.make_token(Question)),
                ':' => Ok(self.make_token(Colon)),
                '"' => self.string(),
                d if d.is_ascii_digit() => Ok(self.number()),
                a if a.is_ascii_alphabetic() => Ok(self.identifier()),
                _ => Err("Unexpected character."),
            }
        }
    }

    #[inline(always)]
    fn get_token_type(
        &mut self,
        expected: char,
        matched: TokenType,
        unmatched: TokenType,
    ) -> TokenType {
        if self.match_char(expected) {
            matched
        } else {
            unmatched
        }
    }

    #[inline(always)]
    pub fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    pub fn make_token(&self, token_type: TokenType) -> Token<'source_lifetime> {
        Token {
            token_type,
            lexeme: &self.source[{ self.start }..{ self.current }],
            line: self.line,
        }
    }

    #[inline(always)]
    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.as_bytes()[self.current - 1] as char
    }

    #[inline(always)]
    pub fn peek(&self) -> char {
        self.source.as_bytes()[self.current] as char
    }

    #[inline(always)]
    pub fn peek_next(&self) -> char {
        self.source.as_bytes()[self.current + 1] as char
    }

    #[inline(always)]
    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn skip_whitespace(&mut self) -> () {
        loop {
            if self.is_at_end() {
                break;
            }
            match self.peek() {
                ' ' => {
                    self.advance();
                }
                '\r' => {
                    self.advance();
                }
                '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    self.skip_comment();
                    return;
                }
                _ => return (),
            }
        }
    }

    fn skip_comment(&mut self) {
        if self.peek_next() == '/' {
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
        } else if self.peek_next() == '*' {
            while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() {
                if self.peek() == '\n' {
                    self.line += 1;
                }
                self.advance();
            }
        } else {
            return ();
        }
    }

    fn string(&mut self) -> Result<Token<'source_lifetime>, &'static str> {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Err("Unterminated string.")
        } else {
            self.advance();
            Ok(self.make_token(StringToken))
        }
    }

    fn number(&mut self) -> Token<'source_lifetime> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.make_token(Number)
    }

    fn identifier(&mut self) -> Token<'source_lifetime> {
        while self.peek().is_ascii_alphabetic() || self.peek().is_ascii_digit() {
            self.advance();
        }
        let it = self.identifier_type();
        self.make_token(it)
    }

    fn identifier_type(&self) -> TokenType {
        match self.source.as_bytes()[self.start] as char {
            'a' => self.check_keyword(1, 2, "nd", And),
            'c' => self.check_keyword(1, 4, "lass", Class),
            'e' => self.check_keyword(1, 3, "lse", Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.as_bytes()[self.start + 1] as char {
                        'a' => self.check_keyword(2, 3, "lse", False),
                        'o' => self.check_keyword(2, 1, "r", For),
                        'u' => self.check_keyword(2, 1, "n", Fun),
                        _ => Identifier,
                    }
                } else {
                    Identifier
                }
            }
            't' => {
                if self.current - self.start > 1 {
                    match self.source.as_bytes()[self.start + 1] as char {
                        'h' => self.check_keyword(2, 2, "is", This),
                        'r' => self.check_keyword(2, 2, "ue", True),
                        _ => Identifier,
                    }
                } else {
                    Identifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", If),
            'n' => self.check_keyword(1, 2, "il", Nil),
            'o' => self.check_keyword(1, 1, "r", Or),
            'p' => self.check_keyword(1, 4, "rint", Print),
            'r' => self.check_keyword(1, 5, "eturn", Return),
            's' => self.check_keyword(1, 4, "uper", Super),
            'v' => self.check_keyword(1, 2, "ar", Var),
            'w' => self.check_keyword(1, 4, "hile", While),
            _ => Identifier,
        }
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length
            && (self.source[{ self.start + start }..{ self.current }].eq(rest))
        {
            token_type
        } else {
            Identifier
        }
    }
}
