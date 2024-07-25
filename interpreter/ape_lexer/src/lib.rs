use std::collections::HashMap;

use ape_ast::{
    Base, LiteralKind,
    TokenType::{self, *},
};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token {
    pub token: TokenType,
    pub len: u32,
    pub lexeme: String,
    pub value: Option<LiteralKind>,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    kwds: HashMap<&'static str, TokenType>,
    line: usize,
    start: usize,
    crnt: usize,
}

impl Lexer {
    pub(crate) fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            kwds: kwds(),
            line: 1,
            start: 0,
            crnt: 0,
        }
    }

    pub(crate) fn lex(&mut self) -> Vec<Token> {
        while !self.is_eof() {
            self.start = self.crnt;
            self.advance_token()
        }
        self.tokens.push(Token {
            token: Eof,
            len: 0,
            lexeme: "\0".to_string(),
            value: None,
            line: self.line,
        });
        self.tokens.clone()
    }

    fn is_eof(&self) -> bool {
        self.crnt >= self.source.len()
    }

    fn advance_token(&mut self) {
        let c = self.advance();
        match c {
            '~' => {
                self.push_token(Tilde, None);
            }
            '%' => {
                self.push_token(Percent, None);
            }
            '(' => {
                self.push_token(LeftParen, None);
            }
            ')' => {
                self.push_token(RightParen, None);
            }
            '{' => {
                self.push_token(LeftBrace, None);
            }
            '}' => {
                self.push_token(RightBrace, None);
            }
            '[' => {
                self.push_token(LeftBracket, None);
            }
            ']' => {
                self.push_token(RightBracket, None);
            }
            ';' => {
                self.push_token(Semi, None);
            }
            ':' => {
                self.push_token(Colon, None);
            }
            ',' => {
                self.push_token(Colon, None);
            }
            '?' => {
                self.push_token(Queston, None);
            }
            '!' => {
                let tt = match self.first() {
                    '!' => NotNot,
                    '=' => NotEq,
                    _ => Not,
                };
                self.push_token(tt, None)
            }
            '&' => {
                let tt = match self.first() {
                    '&' => AndAnd,
                    _ => And,
                };
                self.push_token(tt, None)
            }
            '+' => {
                let tt = match self.first() {
                    '+' => Increment,
                    '=' => PlusEq,
                    _ => Plus,
                };
                self.push_token(tt, None)
            }
            '-' => {
                let tt = match self.first() {
                    '>' => Arrow,
                    '-' => Decr,
                    '=' => MinEq,
                    _ => Minus,
                };
                self.push_token(tt, None)
            }
            '*' => {
                let tt = match self.first() {
                    '*' => Square,
                    '=' => MultEq,
                    _ => Mult,
                };
                self.push_token(tt, None)
            }
            '=' => {
                let tt = match self.first() {
                    '=' => Eq,
                    _ => Assign,
                };
                self.push_token(tt, None)
            }
            '|' => {
                let tt = match self.first() {
                    '|' => Or,
                    _ => Pipe,
                };
                self.push_token(tt, None)
            }
            '.' => {
                let tt = match self.first() {
                    '.' => DotDot,
                    _ => Dot,
                };
                self.push_token(tt, None)
            }
            '<' => {
                let tt = match self.first() {
                    '=' => LessOrEq,
                    _ => Less,
                };
                self.push_token(tt, None)
            }
            '>' => {
                let tt = match self.first() {
                    '=' => GreaterOrEq,
                    _ => Greater,
                };
                self.push_token(tt, None)
            }
            '\\' => {
                let tt = match self.first() {
                    '{' => StartParse,
                    '}' => EndParse,
                    _ => Escape,
                };
                self.push_token(tt, None)
            }
            '/' => {
                if self.first() == '/' {
                    self.comment();
                } else if self.first() == '*' {
                    self.block_comment();
                } else {
                    let tt = match self.first() {
                        '=' => DivEq,
                        _ => Divide,
                    };
                    self.push_token(tt, None)
                }
            }
            ' ' | '\t' | '\r' => {}
            '\n' => self.line += 1,
            '\'' => self.char(),
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(c),
            c if c.is_alphabetic() || c == '_' => self.ident(),
            _ => {
                // @error unknown character: c
            }
        };
    }

    fn comment(&mut self) {
        loop {
            if self.peek() == '\n' || self.is_eof() {
                break;
            }
            self.advance();
        }
    }
    fn block_comment(&mut self) {
        loop {
            if self.peek() == '*' || self.is_eof() {
                self.advance();
                if self.peek() == '/' || self.is_eof() {
                    break;
                }
            }
            self.advance();
        }
    }

    fn char(&mut self) {
        let value = if self.peek() != '\'' && !self.is_eof() {
            let c = self.peek();
            self.advance();
            c
        } else {
            // @error empty or unterminated character
            return;
        };
        if self.peek() != '\'' {
            // @error untermianted character
            return;
        }
        self.advance();
        self.push_token(CharLit, Some(LiteralKind::Char { value }));
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_eof() {
            // @error unterminated string
        }
        self.advance();
        let value = &self.source[self.start + 1..self.crnt - 1];
        self.push_token(StringLit, Some(LiteralKind::String { value }))
    }

    fn ident(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let sub = &self.source[self.start..self.crnt];
        let token = self.kwds.get(sub).clone().unwrap_or(&Ident);
        self.push_token(token.clone(), None);
    }

    fn number(&mut self, c: char) {
        let mut base = Base::Decimal;
        if c == '0' {
            match self.first() {
                'b' => self.parse_binary(),
                'o' => self.parse_octal(),
                'x' => self.parse_hexadecimal(),
                '0'..='9' | '_' | '.' => self.parse_decimal(),
                _ => self.push_token(NumberLit, Some(LiteralKind::Number { base, value: 0.0 })),
            }
        } else {
            self.parse_decimal()
        }
    }

    fn parse_decimal(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let sub = &self.source[self.start..self.crnt];
        let val = sub.parse::<f32>();
        match val {
            Ok(value) => self.push_token(
                NumberIdent,
                Some(LiteralKind::Number {
                    base: Base::Decimal,
                    value,
                }),
            ),
            Err(_) => {
                // @error failed to parse a number: sub
            }
        }
    }

    fn parse_binary(&mut self) {
        while self.peek().is_digit(2) {
            self.advance();
        }
        let sub = &self.source[self.start..self.crnt];
        let val = sub.parse::<f32>();
        match val {
            Ok(value) => self.push_token(
                NumberIdent,
                Some(LiteralKind::Number {
                    base: Base::Binary,
                    value,
                }),
            ),
            Err(_) => {
                // @error failed to parse a binary number: sub
            }
        }
    }

    fn parse_octal(&mut self) {
        while self.peek().is_digit(8) {
            self.advance();
        }
        let sub = &self.source[self.start..self.crnt];
        let val = sub.parse::<f32>();
        match val {
            Ok(value) => self.push_token(
                NumberIdent,
                Some(LiteralKind::Number {
                    base: Base::Octal,
                    value,
                }),
            ),
            Err(_) => {
                // @error failed to parse an octal number: sub
            }
        }
    }

    fn parse_hexadecimal(&mut self) {
        while self.peek().is_digit(16) {
            self.advance();
        }
        let sub = &self.source[self.start..self.crnt];
        let val = sub.parse::<f32>();
        match val {
            Ok(value) => self.push_token(
                NumberIdent,
                Some(LiteralKind::Number {
                    base: Base::Hexadecimal,
                    value,
                }),
            ),
            Err(_) => {
                // @error failed to parse a hexadecimal number: sub
            }
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.crnt).unwrap_or('\0');
        self.crnt += 1;
        c
    }

    fn push_token(&mut self, token: TokenType, value: Option<LiteralKind>) {
        let lexeme = self.source[self.start..self.crnt].to_string();
        self.tokens.push(Token {
            token,
            lexeme: lexeme.clone(),
            line: self.line,
            value,
            len: lexeme.len() as u32,
        })
    }

    fn first(&self) -> char {
        self.source.chars().clone().next().unwrap_or('\0')
    }

    fn second(&self) -> char {
        let mut c = self.source.chars().clone();
        c.next();
        c.next().unwrap_or('\0')
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            return '\0';
        }
        self.source.chars().nth(self.crnt).unwrap()
    }
}

pub fn kwds() -> HashMap<&'static str, TokenType> {
    HashMap::from([
        ("if", If),
        ("else", Else),
        ("else if", ElseIf),
        ("return", Return),
        ("while", While),
        ("loop", Loop),
        ("break", Break),
        ("match", Match),
        ("mod", Mod),
        ("use", Use),
        ("as", As),
        ("from", From),
        ("struct", Struct),
        ("impl", Impl),
        ("enum", Enum),
        ("async", Async),
        ("await", Await),
        ("pub", Pub),
        ("mut", Mut),
        ("func", Func),
        ("number", NumberIdent),
        ("string", StringIdent),
        ("char", CharIdent),
        ("bool", BoolIdent),
        ("null", NullIdent),
        ("void", VoidIdent),
        ("array", ArrayIdent),
        ("any", AnyIdent),
    ])
}
