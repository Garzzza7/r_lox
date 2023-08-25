use std::fmt::Debug;

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
}
impl Default for Scanner{
    fn default() -> Scanner {
        Scanner { source: Vec::new(), tokens: Vec::new(), start: 0, current: 0, line: 1 }
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
            _ => print!(""),
        }
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