#![allow(unused_variables)]
use std::env;
use std::fs;
use std::fmt::{Display, Formatter};
use std::fmt::Error;
use std::str::Chars;

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

enum TokenError {
    UnexpectedCharacter(char),
    UnterminatedString
}

impl Display for TokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
	match self {
	    Self::UnexpectedCharacter(c) => write!(f, "Unexpected character: {c}"),
	    Self::UnterminatedString => write!(f, "Unterminated string.")
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
    EOF,
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
	    TokenType::String(_) => "STRING",
	    TokenType::Number(_, _) => "NUMBER",
	};

	write!(f, "{}", kind)
    }
}


// <TokenType> <lexeme> <literal>
// STRING "lox" lox
#[derive(Debug)]
pub struct Token {
    tokenType: TokenType,
    lexeme: String,
    literal: String,
}



impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
	write!(f, "{} {} {}", self.tokenType, self.lexeme, self.literal)
    }

}

impl Token {
    fn new (token_type: TokenType) -> Self {

	let (literal, lexeme) =  match &token_type {
	    TokenType::String(val)  | TokenType::Number(val, _) =>{
		let lexeme = format!("\"{}\"", &val);
		(val.to_string(), lexeme)
	    }
	    ,
	    TokenType::EOF => (String::from("null"), String::from(" ")),
	    TokenType::Bang =>  (String::from("null"), String::from("!")),
	    TokenType::BangEqual => (String::from("null"), String::from("!=")),
	    TokenType::Comma => (String::from("null"), String::from(",")),
	    TokenType::Dot => (String::from("null"), String::from(".")),
	    TokenType::Equal => (String::from("null"), String::from("=")),
	    TokenType::EqualEqual => (String::from("null"), String::from("==")),
	    TokenType::Greater => (String::from("null"), String::from(">")),
	    TokenType::GreaterEqual => (String::from("null"), String::from(">=")),
	    TokenType::Less => (String::from("null"), String::from("<")),
	    TokenType::LessEqual => (String::from("null"), String::from(">")),
	    TokenType::LeftBrace => (String::from("null"), String::from("}")),
	    TokenType::RightBrace => (String::from("null"), String::from("{")),
	    TokenType::LeftParen => (String::from("null"), String::from("(")),
	    TokenType::RightParen => (String::from("null"), String::from(")")),
	    TokenType::Minus => (String::from("null"), String::from("-")),
	    TokenType::Plus => (String::from("null"), String::from("+")),
	    TokenType::Star => (String::from("null"), String::from("*")),
	    TokenType::Semicolon => (String::from("null"), String::from(";")),
	    TokenType::Slash => (String::from("null"), String::from("/"))
	};
	
	Self {
	    tokenType: token_type,
	    lexeme: lexeme,
	    literal: literal
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

    fn peek(&mut self) -> Option<char> {
	if self.is_at_end() {
	    return None;
	}
	self.source[self.current..].chars().next()
    }

    pub fn add_token(&mut self, token: Token) {
	self.tokens.push(token);
    }

    pub fn tokenize(&mut self) {

	let mut is_err = false;

	loop {
	    match self.advance() {
		Some('(') => self.add_token(Token::new(TokenType::LeftParen)),
		Some(')') => self.add_token(Token::new(TokenType::RightParen)),
		Some('}') => self.add_token(Token::new(TokenType::LeftBrace)),
		Some('{') => self.add_token(Token::new(TokenType::RightBrace)),
		Some('+') => self.add_token(Token::new(TokenType::Plus)),
		Some('*') => self.add_token(Token::new(TokenType::Star)),
		Some(',') => self.add_token(Token::new(TokenType::Comma)),
		Some('.') => self.add_token(Token::new(TokenType::Dot)),
		Some(';') => self.add_token(Token::new(TokenType::Semicolon)),
		Some('-') => self.add_token(Token::new(TokenType::Minus)),
		Some('=') => if let Some('=') = self.peek() {
		    self.advance();
		    self.add_token(Token::new(TokenType::EqualEqual));
		} else {
		    self.add_token(Token::new(TokenType::Equal));
		}
		,
		Some('!') => if let Some('=') = self.peek() {
		    self.advance();
		    self.add_token(Token::new(TokenType::BangEqual));
		} else {
		    self.add_token(Token::new(TokenType::Bang));
		},
		Some('<') => if let Some('=') = self.peek() {
		    self.advance();
		    self.add_token(Token::new(TokenType::LessEqual));
		} else {
		    self.add_token(Token::new(TokenType::Minus))
		},
		Some('>') => if let Some('=') = self.peek() {
		    self.advance();
		    self.add_token(Token::new(TokenType::GreaterEqual));
		} else {
		    self.add_token(Token::new(TokenType::Greater));
		},
		Some('/') => if let Some('/') = self.peek() {
		    while self.peek() != Some('\n') && !self.is_at_end() {
			self.advance();
		    }
		    continue;
		} else {
		    self.add_token(Token::new(TokenType::Slash));
		},
		Some(' ') | Some('\t') | Some('\n') =>
		    continue,

		Some('"') =>
		{
		    let mut dummy: String = String::from("");
		    loop {
			match self.peek() {
			    Some('"') => {
				self.add_token(Token::new(TokenType::String(dummy)));
				self.advance();
				break;
			    },
			    Some('\n') | None => {
				is_err = true;
				eprintln!("[line {}] Error: Unterminated string", self.line);
				self.advance();
				break;
			    },
			    Some(a)  => {
				dummy.push(a);
				self.advance();
			    },
			    
			}
		    }
		}
		,
		None => self.add_token(Token::new(TokenType::EOF)),
		Some(c) => 
		{
		    is_err = true;
		    eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
		    break;
		}
	    }

	}

	if is_err {
	    std::process::exit(65);
	}

    }
}

