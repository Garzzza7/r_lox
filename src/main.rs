use std::{env, str::Bytes, fs, io};
use input_stream::InputStream;
fn main() {
    let args: Vec<_> = env::args().collect();
    //println!("ROZMIAR = {}",args.len());
    if args.len() > 1 {
        println!("Usage: rlox [script]");
    }else if args.len() == 1 {
        run_file(&args[0]);
    }else{
        run_prompt();
    }
}
fn run_file(_path : &String) -> () {
    //println!("ty pierdolony debilu {}",_path);
    let bytes = fs::read_to_string(_path)
    .expect("Should have been able to read the file");
    run(&bytes);
}
fn run_prompt() -> () {

    loop{
        let mut stdin = io::stdin();
        let mut line = String::with_capacity(2048);
        print!("> ");
        stdin.read_line(&mut line);
        println!("PIERDOLONY INPUT = {}",line);
        //run(&line);
        line.clear();
    }
}
fn run(source : &String) -> () {

}

