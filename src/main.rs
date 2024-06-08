//use std::arch::x86_64::_mm512_scalef_round_pd;

mod scanner;
use crate::scanner::*;
use std::fs::{self, File};
use std::io::prelude::*;
use std::{env, io};

static mut HAD_ERROR: bool = false;

fn main() {
    let args: Vec<_> = env::args().collect();
    //println!("ROZMIAR = {}", args.len());
    if args.len() > 2 {
        println!("Pass --help if you want to know what rlox is capable of.");
        std::process::exit(64);
    } else if args.len() == 2 {
        //println!("TUTAJ PATZRZ {}",args[1]);
        /* let buffer = &args[1] */
        if fs::metadata(&args[1]).is_ok() {
            run_file(&args[1]);
        } else {
            println!("Given file does not exist.\nAborting.");
        }
    } else {
        run_prompt();
    }
}

fn run_file(_path: &String) -> () {
    let mut file = File::open(_path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    //println!("{}", contents);
    run(&contents);
    unsafe {
        if HAD_ERROR {
            std::process::exit(0);
        }
    }
}

fn run_prompt() -> () {
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        print!("rlox> ");
        stdin.read_line(&mut line);
        if line.is_empty() == true {
            break;
        }
        run(&line);
        line.clear();
        unsafe {
            HAD_ERROR = false;
        }
    }
}

fn run(_source: &String) -> () {
    unsafe {
        if HAD_ERROR == true {
            std::process::exit(65);
        }
    }
    let tokens = scan_tokens(_source.to_string());
    //println!("{}", _source.to_string());
    //let tokens: Vec<Token> = scanner.tokens;
    //let token = _source.split_ascii_whitespace();
    //let token = _source.as_bytes();
    for _i in tokens {
        println!("{:?}", _i);
    }
}

#[warn(dead_code)]
fn error(line: i32, message: &String) -> () {
    let wher: String = String::from("");
    report(line, &wher, &message);
}

#[warn(dead_code)]
fn report(line: i32, wher: &String, message: &String) -> () {
    println!("[line {} ] Error {} : {}", line, wher, message);
    unsafe {
        HAD_ERROR = true;
    }
}
