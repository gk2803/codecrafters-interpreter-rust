#![allow(unused_variables)]
use std::env;
use std::fs;
use std::fmt::{Display, Formatter};
use std::fmt::Error;
use std::str::Chars;
mod lexer;
use lexer::lexer::Lexer;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

	    let mut lexer = Lexer::new(&file_contents);

	    


            if !file_contents.is_empty() {
		lexer.tokenize();
		
            } else {
		println!("EOF  null");
	    }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}












