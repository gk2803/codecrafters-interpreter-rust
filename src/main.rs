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
#[derive(Debug)]
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

	
	let kind = match self {
	    TokenType::EOF => "EOF",
	    TokenType::LeftParen => "LEFT_PAREN",
	    TokenType::RightParen => "RIGHT_PAREN",
	    TokenType::LeftBrace => "LEFT_BRACE",
	    TokenType::RightBrace => "RIGHT_BRACE",
	    TokenType::Comma => "COMMA",
	    TokenType::Dot => "DOT",
	    TokenType::Star => "STAR",
	    TokenType::Plus => "PLUS",
	    TokenType::Minus => "MINUS",
	    TokenType::Semicolon => "SEMICOLON",
	    TokenType::Slash => "SLASH",
	    TokenType::Bang => "BANG",
	    TokenType::BangEqual => "BANG_EQUAL",
	    TokenType::Greater => "GREATER",
	    TokenType::GreaterEqual => "GREATER_EQUAL",
	    TokenType::Less => "LESS",
	    TokenType::LessEqual => "LESS_EQUAL",
	    TokenType::Equal => "EQUAL",
	    TokenType::EqualEqual => "EQUAL_EQUAL",
	    TokenType::String(val) => &format!("STRING \"{}\" {}", val, val),
	    TokenType::Number(raw, val) => &format!("NUMBER {} {}", raw, val),
	};

	write!(f, "{}", kind)
    }
}

#[derive(Debug)]
pub struct Token {
    tokenType: TokenType,
    lexeme: Option<String>,
    literal: String,
}



impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
	let lexeme = self.lexeme.as_deref().unwrap_or("null");
	if self.literal.is_empty() {
	    write!(f, "{}  {}", self.tokenType, lexeme)
	} else {
	    write!(f, "{} {} {}", self.tokenType, self.literal, lexeme)
	}
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
		Some('}') => Token {
		    tokenType: TokenType::RightBrace,
		    lexeme: None,
		    literal: "}".to_string()
		} ,
		Some('{') => Token {
		    tokenType: TokenType::LeftBrace,
		    lexeme: None,
		    literal: "{".to_string()
		},
		Some('+') => Token {
		    tokenType: TokenType::Plus,
		    lexeme: None,
		    literal: "+".to_string()
		},
		Some('*') => Token {
		    tokenType: TokenType::Star,
		    lexeme: None,
		    literal: "*".to_string()
		},
		Some('/') => Token {
		    tokenType: TokenType::Slash,
		    lexeme: None,
		    literal: "".to_string()
		},
		Some(',') => Token {
		    tokenType: TokenType::Comma,
		    lexeme: None,
		    literal: ",".to_string()
		},
		Some('.') => Token {
		    tokenType: TokenType::Dot,
		    lexeme: None,
		    literal: ".".to_string()
		},
		Some(';') => Token {
		    tokenType: TokenType::Semicolon,
		    lexeme: None,
		    literal: ";".to_string()
		},
		Some('-') => Token {
		    tokenType: TokenType::Minus,
		    lexeme: None,
		    literal: "-".to_string()
		},
		None => 
			Token {
			    tokenType: TokenType::EOF,
			    lexeme: None,
			    literal: "".to_string()
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

