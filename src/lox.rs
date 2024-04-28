use std::{io::{self, BufRead, Write}, process::exit, sync::Mutex};

use lazy_static::lazy_static;

use crate::scanner::scanner::Scanner;

lazy_static! {
    static ref LOX: Mutex<Lox> = Mutex::new(Lox::new());
}

pub struct Lox {
    had_error: bool
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false
        }
    }

    pub fn run_file(&mut self, path: String) -> Result<(), io::Error> {
        let content = std::fs::read_to_string(path)?;
        
        self.run(content);
        
        if self.had_error {
            exit(1);
        }
        Ok(())
    }
    
    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let line = {
                let result = io::stdin()
                .lock()
                .lines()
                .next();
                // When user enter Ctrl+D
                if let None = result {
                    println!();
                    break;
                }
                result.unwrap().unwrap()
            };
            self.run(line);
            self.had_error = false;
        }
    }
    
    fn run(&mut self, code: String) {
        let mut scanner = Scanner::new(code);

        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token.to_str());
        }
    }

    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, String::from(""), message);
    }

    fn report(&mut self, line: usize, location: String, message: String) {
        println!("[line {line}] Error{location}: {message}");
        self.had_error = true;
    }

    pub fn report_error(line: usize, message: String) {
        LOX.lock().unwrap().error(line, message);
    }
}