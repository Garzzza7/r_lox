use std::any::{Any, TypeId};
//use std::arch::x86_64::_mm512_scalef_round_pd;
use std::os::unix::process;
use std::ptr::{null, self};
use std::{env, fs, io};
use std::io::prelude::*;
use std::fs::File;
static mut hadError: bool = false;
enum TokenType {
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
struct Token<'a> {
    ttype: TokenType,
    lexeme: String,
    literal:  &'a dyn Any,
    line: i32,

}
impl Token<'_>{
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
struct Scanner<'a> {
    source: String,
    tokens: Vec<Token<'a>>,
    start: i32,
    current:i32,
    line:i32,
}
impl Scanner<'static> {
    fn scanTokens(mut self) -> Vec<Token<'static>>{
        loop{
            if(!self.isAtEnd()){
                break;
            }
            self.start=self.current;
            self.scanToken();       
        }
        let lit: &dyn Any=&"";
       // let new_token: Token = Token { ttype: TokenType::EOF, lexeme: String::from(""), literal: lit, line: self.line };
        self.tokens.push( Token { ttype: TokenType::EOF, lexeme: String::from(""), literal: lit, line: self.line });
        self.tokens
        //let mut newvec: Vec<Token>=Vec::new();
        //newvec=self.tokens;
        //newvec
    }
    fn isAtEnd(&self)->bool{
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
fn main() {
    let args: Vec<_> = env::args().collect();
    //println!("ROZMIAR = {}",args.len());
    if args.len() > 2 {
        println!("Usage: rlox [script]");
    }else if args.len() == 2 {
        //println!("TUTAJ PATZRZ {}",args[1]);
        let mut buffer = &args[1];
        run_file(buffer);
    }else{
        run_prompt();
    }
}

fn run_file(_path : &String) -> () {
    let mut file = File::open(_path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    //println!("GOWNO w plynie skurwysynie {}", contents);
    run(&contents);
    unsafe{
        if(hadError){
            std::process::exit(0);
        }
    }
}
fn run_prompt() -> () {
    loop{
        print!("> ");
        let stdin = io::stdin();
        let mut line = String::with_capacity(2048);
        stdin.read_line(&mut line);
        run(&line);
        unsafe{
            hadError=false;
        }
        line.clear();
    }
}
fn run(_source : &String) -> () {
    //println!("HELLO FROM RUN");

    //let stream: proc_macro::TokenStream = _source.parse()
    let mut s:String =_source.to_string();
    let mut bruh = s.split(" ");
    for i in bruh {
        println!("{}",i);
    }
}

fn error(line : i32, message: &String) -> (){
    let s: String = "".to_string();
    report(line, &s, &message);
}
fn report(line: i32, w: &String , message: &String) ->() {
    println!("[line {} ] Error {} : {}",line,w,message);
    unsafe{
    hadError = true;
    }
}
