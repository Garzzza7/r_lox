use std::{fmt::Debug, f32::consts::E};
use std::collections::HashMap;
#[derive(Debug,Clone)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,
    // Literals.
    IDENTIFIER, STRING, NUMBER,
    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
    EOF
}
#[derive(Debug,Clone)]
pub enum Literal {
    Indentifier(String),
    Str(String),
    Number(f64),
}
#[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Option<Literal>,
    pub line: usize,

}
impl Debug for Token{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"Token{{ ttype = {:?}, lexeme = {:?}, literal = {:?}, line = {:?}}}",
            self.ttype,
            self.lexeme,
            self.literal,
            self.line
        )
    }
}

struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords:HashMap<String,TokenType>,
}
impl Default for Scanner{
    fn default() -> Scanner {
        Scanner { source: Vec::new(), tokens: Vec::new(), start: 0, current: 0, line: 1 ,keywords:
                vec![
                ("and", TokenType::AND),
                ("class", TokenType::CLASS),
                ("else", TokenType::ELSE),
                ("false", TokenType::FALSE),
                ("for", TokenType::FOR),
                ("fun", TokenType::FUN),
                ("if", TokenType::IF),
                ("nil", TokenType::NIL),
                ("or", TokenType::OR),
                ("print", TokenType::PRINT),
                ("return", TokenType::RETURN),
                ("super", TokenType::SUPER),
                ("this", TokenType::THIS),
                ("true", TokenType::TRUE),
                ("var", TokenType::VAR),
                ("while", TokenType::WHILE),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect(),

        }
    }
}
impl Scanner {
    fn scanTokens(&mut self){
        loop{
            if !self.isAtEnd() {break;}
            self.start=self.current;
            self.scanToken();       
        }
   //     let lit: &dyn Any=&"";
       // let new_token: Token = Token { ttype: TokenType::EOF, lexeme: String::from(""), literal: lit, line: self.line };
        self.tokens.push( Token { ttype: TokenType::EOF, lexeme: Vec::new(), literal: None, line: self.line });
        //self.tokens
        //let mut newvec: Vec<Token>=Vec::new();
        //newvec=self.tokens;
        //newvec
    }
    fn isAtEnd(&mut self)->bool{
        self.current>=self.source.len().try_into().unwrap()
    }
    fn scanToken(&mut self) -> () {
        let c:char=self.advance();
        match c {
            '(' => self.addToken(TokenType::LEFT_PAREN),
            ')' => self.addToken(TokenType::RIGHT_PAREN),
            '{' => self.addToken(TokenType::LEFT_BRACE),
            '}' => self.addToken(TokenType::RIGHT_BRACE),
            ',' => self.addToken(TokenType::COMMA),
            '.' => self.addToken(TokenType::DOT),
            '-' => self.addToken(TokenType::MINUS),
            '+' => self.addToken(TokenType::PLUS),
            ';' => self.addToken(TokenType::SEMICOLON),
            '*' => self.addToken(TokenType::STAR),
            '!' =>{ 
                    if self.matches('='){
                    self.addToken(TokenType::BANG_EQUAL);
                    }else{
                        self.addToken(TokenType::BANG);
                    } 
                }
            '='=>{ 
                    if self.matches('='){
                    self.addToken(TokenType::EQUAL_EQUAL);
                    }else{
                        self.addToken(TokenType::EQUAL);
                    } 
                }
            '<'=>{ 
                    if self.matches('='){
                    self.addToken(TokenType::LESS_EQUAL);
                    }else{
                        self.addToken(TokenType::LESS);
                    } 
                }
            '>'=>{ 
                    if self.matches('='){
                    self.addToken(TokenType::GREATER_EQUAL);
                    }else{
                        self.addToken(TokenType::GREATER);
                    } 
                }
            '/'=>{  
                if(self.matches('/')){
                    while self.peek()!='\n' && !self.isAtEnd() {
                        self.advance();
                    }
                }else{
                    self.addToken(TokenType::SLASH);
                }
                }
            ' '=>{

            } 
            '\r'=>{

            }
            '\t'=>{

            }
            '\n'=>{
                self.line+=1;
            }
            '"'=>{
                self.string();
            }
            'o'=>{
                if self.peek()=='r' {
                    self.addToken(TokenType::OR);
                }
            }
            _ => {
                if(Scanner::isDigit(c)){
                    self.number();
                }else if  Scanner::isAlpha(c){
                    self.indentifier();
                }else{
                    print!("Unexpected character at line {}",self.line);
                }
            }
        }
    }
    fn isAlphaNumeric(c:char)->bool{
        Scanner::isAlpha(c) || Scanner::isDigit(c)
    }
    fn isAlpha(c:char)->bool{
        (c>='a' && c<='z') || (c>='A' && c<='Z') || c=='_'
    }
    fn indentifier(&mut self)->(){
        while Scanner::isAlphaNumeric(self.peek()) {
            self.advance();
        }
        
        let val = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();

        let token_type: TokenType = match self.keywords.get(&val) {
            Some(kw_token_type) => kw_token_type.clone(),
            None => TokenType::IDENTIFIER,
        };

        match token_type {
            TokenType::IDENTIFIER => self.addTokenLiteral(
                TokenType::IDENTIFIER,
                Some(Literal::Indentifier(val)),
            ), // book doesn't do this. why not?}
            _ => self.addToken(token_type),
        }
        
        self.addToken(TokenType::IDENTIFIER);
    }
    fn isDigit(c:char)->bool{
        c>='0' && c<='9'
    }
    fn number(&mut self)->(){
        while Scanner::isDigit(self.peek()){
            self.advance();
        }
        if(self.peek()=='.'&&Scanner::isDigit(self.peekNext())){
            self.advance(); 
            while Scanner::isDigit(self.peek()) {
                self.advance();
            }
        }
        let val: f64 = String::from_utf8(self.source[self.start..self.current].to_vec())
        .unwrap()
        .parse()
        .unwrap();
        self.addTokenLiteral(TokenType::NUMBER,Some(Literal::Number(val)));
    }
    fn peekNext(&mut self)->char{
        if self.current+1>=self.source.len(){
            return '\0';
        }
        char::from(self.source[self.current+1])
    }
    fn string(&mut self) -> (){
       while self.peek()!='"' && !self.isAtEnd() {
        if(self.peek()=='\n'){
            self.line+=1;
        }
        if(self.isAtEnd()){
            println!("Unterminated string at line {}",self.line);
        }
        self.advance();
        let value = self.source[self.start+1..self.current-1].to_vec();
        self.addTokenLiteral(TokenType::STRING,Some(Literal::Str(String::from_utf8(value).unwrap())));
       } 
    }
    fn peek(&mut self) -> char{
        if(self.isAtEnd()){
            return '\0';
        }
        return char::from(self.source[self.current]);
    }
    fn matches(&mut self,expected:char) -> bool{
        if self.isAtEnd() {
            return false;
        }   
        if char::from(self.source[self.current]) != expected {
            return false;
        }
        self.current+=1;
        true
    }
    fn addToken(&mut self, tt: TokenType) -> () {
        self.addTokenLiteral(tt,None)
    }
    fn addTokenLiteral(&mut self, tt: TokenType, literal: Option<Literal>) -> (){
       let text = self.source[self.start..self.current].to_vec();
       self.tokens.push(Token { ttype: tt, lexeme: text, literal: literal, line: self.line })
    }
    //fn addToken(tt: TokenType, literal: &dyn Any) -> () {
    //    Self::addToken(tt)
    //}


    fn advance(&mut self) -> char {
        self.current+=1;

        char::from(self.source[self.current-1])
        //let my_vec: Vec<char> = self.source.chars().collect();
        //let index = usize::try_from(self.current).unwrap();
        //my_vec[index-1]
    }
}