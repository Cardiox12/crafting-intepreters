use std::{process::exit, sync::Mutex};

pub mod scanner;
mod lox;

use lazy_static::lazy_static;
use lox::Lox;

lazy_static! {
    static ref LOX: Mutex<Lox> = Mutex::new(Lox::new());
}

fn main() {
    let args = std::env::args();
    let mut lox = Lox::new();

    if args.len() > 2 { 
        println!("Usage: rlox [script]");
        exit(1);
    } else if args.len() == 2 {
        let path = std::env::args().nth(1).unwrap();
        lox.run_file(path).unwrap();
    } else {
        lox.run_prompt();
    }
}
