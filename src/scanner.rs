

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
pub enum Literal {
    Indentifier(String),
    Str(String),
    Number(f64),
}
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,

}
impl Token{
   fn toString(&self) -> String {
        let mut s: String = String::from("");
        //s+=&self.ttype;
        s+=&" ".to_string();
        s+=&self.lexeme;
        s+=&" ".to_string();
        //s+=&String::from(self.literal);
        s
   } 
}
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current:i32,
    line:i32,
}
impl Scanner {
    fn scanTokens(mut self) -> Vec<Token>{
        loop{
            if(!self.isAtEnd()){
                break;
            }
            self.start=self.current;
            self.scanToken();       
        }
   //     let lit: &dyn Any=&"";
       // let new_token: Token = Token { ttype: TokenType::EOF, lexeme: String::from(""), literal: lit, line: self.line };
       // self.tokens.push( Token { ttype: TokenType::EOF, lexeme: String::from(""), literal: lit, line: self.line });
        self.tokens
        //let mut newvec: Vec<Token>=Vec::new();
        //newvec=self.tokens;
        //newvec
    }
    fn isAtEnd(&mut self)->bool{
        self.current>=self.source.len().try_into().unwrap()
    }
    fn scanToken(&mut self) -> () {
        let mut c:char=self.advance();
        match c {
            '(' => Self::addToken(TokenType::LEFT_PAREN),
            ')' => Self::addToken(TokenType::RIGHT_PAREN),
            '{' => Self::addToken(TokenType::LEFT_BRACE),
            '}' => Self::addToken(TokenType::RIGHT_BRACE),
            ',' => Self::addToken(TokenType::COMMA),
            '.' => Self::addToken(TokenType::DOT),
            '-' => Self::addToken(TokenType::MINUS),
            '+' => Self::addToken(TokenType::PLUS),
            ';' => Self::addToken(TokenType::SEMICOLON),
            '*' => Self::addToken(TokenType::STAR),
            _ => print!(""),
        }
    }
    fn addToken(tt: TokenType) -> () {
        Self::addToken(tt)
    }
    //fn addToken(tt: TokenType, literal: &dyn Any) -> () {
    //    Self::addToken(tt)
    //}


    fn advance(&mut self) -> char {
        self.current+=1;
        let my_vec: Vec<char> = self.source.chars().collect();
        let index = usize::try_from(self.current).unwrap();
        my_vec[index-1]
    }
}