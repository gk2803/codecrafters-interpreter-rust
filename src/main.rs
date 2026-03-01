#![allow(unused_variables)]
use std::env;
use std::fs;
use std::fmt::{Display, Formatter};
use std::fmt::Error;


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
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

	    let mut lexer = Lexer::new(&file_contents);
	    lexer.tokenize();
	    


            if !file_contents.is_empty() {
		lexer.tokenize();
		
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    String(String), Number(String, f64),

    // Keywords.


    EOF
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
	match self {
	    TokenType::EOF => write!(f, ""),
	    TokenType::LeftParen => write!(f, "LEFT_PAREN"),
	    TokenType::RightParen => write!(f, "RIGHT_PAREN"),
	    TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
	    TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
	    TokenType::Comma => write!(f, "COMMA"),
	    TokenType::Dot => write!(f, "DOT"),
	    TokenType::Star => write!(f, "STAR"),
	    TokenType::Plus => write!(f, "PLUS"),
	    TokenType::Minus => write!(f, "MINUS"),
	    TokenType::Semicolon => write!(f, "SEMICOLON"),
	    TokenType::Slash => write!(f, "SLASH"),
	    TokenType::Bang => write!(f, "BANG"),
	    TokenType::BangEqual => write!(f, "BANG_EQUAL"),
	    TokenType::Greater => write!(f, "GREATER"),
	    TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
	    TokenType::Less => write!(f, "LESS"),
	    TokenType::LessEqual => write!(f, "LESS_EQUAL"),
	    TokenType::Equal => write!(f, "EQUAL"),
	    TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
	    TokenType::String(val) => write!(f, "STRING \"{}\" {}", val, val),
	    TokenType::Number(raw, val) => write!(f, "NUMBER {} {}", raw, val),
	}
    }
}

pub struct Token {
    tokenType: TokenType,
    lexeme: Option<String>,
    literal: String,
}



impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
	let lexeme = self.lexeme.as_deref().unwrap_or("null");
	write!(f, "{} {} {}", self.tokenType, self.literal, lexeme)
    }
}



pub struct Lexer<'a> {
    source: &'a String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}


impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
	Self {
	    source,
	    tokens: Vec::new(),
	    start: 0,
	    current: 0,
	    line: 1
	}
    }

    fn is_at_end(&self) -> bool {
	self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
	if self.is_at_end() {
	    return None;
	}

	let c = self.source[self.current..].chars().next()?;
	self.current += c.len_utf8();
	if c == '\n' {
	    self.line += 1;
	}
	Some(c)
    }

    pub fn tokenize(&mut self) {
	loop {
	    let tok = match self.advance() {
		Some('(') => 
		    Token {
			tokenType: TokenType::LeftParen,
			lexeme: None,
			literal: "(".to_string()
		    }
		,
		Some(')') => 
		    Token {
			tokenType: TokenType::RightParen,
			lexeme: None,
			literal: ")".to_string()
		    }
		,
		None => 
			Token {
			    tokenType: TokenType::EOF,
			    lexeme: None,
			    literal: "EOF".to_string()
			}
		,
		_ => continue
	    };

	    let is_eof = matches!(tok.tokenType, TokenType::EOF);
	    self.tokens.push(tok);
	    println!("{}", self.tokens.last().unwrap());
	    if is_eof {
		break;
	    }
	}
    }
}

