pub mod lib;
pub mod util;
use std::env;
use std::io::{stdin, stdout, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(args[1].as_str()),
        _ => println!("Usage: lexblix [file]"),
    }
}

fn run_prompt() {
    let mut s = String::new();
    loop {
        print!("Please enter some text: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        run(&s);
        // println!("You typed: {}",s);
        s.clear();
    }
}

fn run_file(file_path: &str) {
    print!("File path: {}", file_path);
}

fn run(source: &String) {
    let mut scanner = lib::scanner::Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("{}", token);
    }
}
