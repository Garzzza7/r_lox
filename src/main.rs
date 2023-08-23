use std::{env, fs, io};
use std::io::prelude::*;
use std::fs::File;
static mut hadError: bool = false;

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
}
fn run_prompt() -> () {
    loop{
        let stdin = io::stdin();
        let mut line = String::with_capacity(2048);
        print!("> ");
        stdin.read_line(&mut line);
        run(&line);
        line.clear();
    }
}
fn run(_source : &String) -> () {
    println!("HELLO FROM RUN");

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
