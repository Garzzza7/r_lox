use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::ptr::null;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,

    //Nothing,
    NULL,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Indentifier(Vec<u8>),
    Str(Vec<u8>),
    Number(f64),
}

#[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Default for Token {
    fn default() -> Token {
        Token {
            ttype: TokenType::NULL,
            lexeme: Vec::new(),
            literal: None,
            line: 1,
        }
    }
}

// impl Display for Option<Literal> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", &self)
//     }
// }
//
// impl Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(
//             f,
//             "Token{{ ttype = {}, lexeme = {}, literal = {}, line = {}}}",
//             self.ttype, self.lexeme, self.literal, self.line
//         )
//     }
// }

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Token{{ ttype = {:?}, lexeme = {:?}, literal = {:?}, line = {:?}}}",
            self.ttype, self.lexeme, self.literal, self.line
        )
    }
}

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<Vec<u8>, TokenType>,
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and".to_string().into_bytes(), TokenType::AND),
                ("class".to_string().into_bytes(), TokenType::CLASS),
                ("else".to_string().into_bytes(), TokenType::ELSE),
                ("false".to_string().into_bytes(), TokenType::FALSE),
                ("for".to_string().into_bytes(), TokenType::FOR),
                ("fun".to_string().into_bytes(), TokenType::FUN),
                ("if".to_string().into_bytes(), TokenType::IF),
                ("nil".to_string().into_bytes(), TokenType::NIL),
                ("or".to_string().into_bytes(), TokenType::OR),
                ("print".to_string().into_bytes(), TokenType::PRINT),
                ("return".to_string().into_bytes(), TokenType::RETURN),
                ("super".to_string().into_bytes(), TokenType::SUPER),
                ("this".to_string().into_bytes(), TokenType::THIS),
                ("true".to_string().into_bytes(), TokenType::TRUE),
                ("var".to_string().into_bytes(), TokenType::VAR),
                ("while".to_string().into_bytes(), TokenType::WHILE),
            ]),
            //.into_iter()
            //.map(|(k, v)| (String::from(k), v))
            //.collect(),
        }
    }
}

impl Scanner {
    pub fn scan_tokens(&mut self) {
        loop {
            if !self.is_at_end() {
                break;
            }
            self.start = self.current;
            self.scan_token();
        }
        //     let lit: &dyn Any=&"";
        // let new_token: Token = Token { ttype: TokenType::EOF, lexeme: String::from(""), literal: lit, line: self.line };
        self.tokens.push(Token {
            ttype: TokenType::EOF,
            lexeme: Vec::new(),
            literal: None,
            line: self.line,
        });
        //self.tokens
        //let mut newvec: Vec<Token>=Vec::new();
        //newvec=self.tokens;
        //newvec
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
        //self.current >= self.source.len().try_into().unwrap()
    }

    fn scan_token(&mut self) -> () {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BANG_EQUAL);
                } else {
                    self.add_token(TokenType::BANG);
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EQUAL_EQUAL);
                } else {
                    self.add_token(TokenType::EQUAL);
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LESS_EQUAL);
                } else {
                    self.add_token(TokenType::LESS);
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GREATER_EQUAL);
                } else {
                    self.add_token(TokenType::GREATER);
                }
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string();
            }
            'o' => {
                if self.peek() == 'r' {
                    self.add_token(TokenType::OR);
                }
            }
            _ => {
                if Scanner::is_digit(c) {
                    self.number();
                } else if Scanner::is_alpha(c) {
                    self.indentifier();
                } else {
                    println!("Unexpected character {} at line {}", c, self.line);
                }
            }
        }
    }

    fn is_alpha_numeric(c: char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn indentifier(&mut self) -> () {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let literal_val = self.source[self.start..self.current].to_vec();

        let token_type = match self.keywords.get(&literal_val) {
            Some(tt) => *tt,
            None => TokenType::IDENTIFIER,
        };

        match token_type {
            TokenType::IDENTIFIER => self.add_token_literal(
                TokenType::IDENTIFIER,
                Some(Literal::Indentifier(literal_val)),
            ), // book doesn't do this. why not?
            _ => self.add_token(token_type),
        }
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) -> () {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance();
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }
        //TODO: Check this
        let val: f64 = String::from_utf8(self.source[self.start..self.current].to_vec())
            .unwrap()
            .parse()
            .unwrap();
        self.add_token_literal(TokenType::NUMBER, Some(Literal::Number(val)));
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        char::from(self.source[self.current + 1])
    }

    fn string(&mut self) -> () {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            if self.is_at_end() {
                println!("Unterminated string at line {}", self.line);
            }
            self.advance();
            let value: Vec<u8> = self.source[self.start + 1..self.current - 1].to_vec();
            //TODO: make sure that this literal actually works
            self.add_token_literal(TokenType::STRING, Some(Literal::Str(value)));
        }
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return char::from(self.source[self.current]);
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if char::from(self.source[self.current]) != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&mut self, tt: TokenType) -> () {
        self.add_token_literal(tt, None);
    }

    fn add_token_literal(&mut self, tt: TokenType, lit: Option<Literal>) -> () {
        let text: Vec<u8> = self.source[self.start..self.current].to_vec();
        self.tokens.push(Token {
            ttype: tt,
            lexeme: text,
            literal: lit,
            line: self.line,
        })
    }

    //fn addToken(tt: TokenType, literal: &dyn Any) -> () {
    //    Self::addToken(tt)
    //}

    fn advance(&mut self) -> char {
        self.current += 1;
        char::from(self.source[self.current - 1])
        //let my_vec: Vec<char> = self.source.chars().collect();
        //let index = usize::try_from(self.current).unwrap();
        //my_vec[index-1]
    }
}
