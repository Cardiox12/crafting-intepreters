use std::{io::{self, BufRead, Write}, process::exit};

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
        println!("{code}");
    }

    pub fn error(&mut self, line: usize, message: String) {
        self.report(line, String::from(""), message);
    }

    fn report(&mut self, line: usize, location: String, message: String) {
        println!("[line {line}] Error{location}: {message}");
        self.had_error = true;
    }
}