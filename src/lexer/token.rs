use super::token_type::TokenType;
use std::fmt::{Display, Formatter, Error};

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
}



impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
	write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }

}

impl Token {
    pub fn new (token_type: TokenType) -> Self {

	let (literal, lexeme) =  match &token_type {
	    TokenType::String(val)  | TokenType::Number(val, _) =>{
		let lexeme = format!("\"{}\"", &val);
		(val.to_string(), lexeme)
	    }
	    ,
	    TokenType::EOF => {
		(String::from("null"), String::from(""))
	    }
	    ,
	    TokenType::Bang =>  (String::from("null"), String::from("!")),
	    TokenType::BangEqual => (String::from("null"), String::from("!=")),
	    TokenType::Comma => (String::from("null"), String::from(",")),
	    TokenType::Dot => (String::from("null"), String::from(".")),
	    TokenType::Equal => (String::from("null"), String::from("=")),
	    TokenType::EqualEqual => (String::from("null"), String::from("==")),
	    TokenType::Greater => (String::from("null"), String::from(">")),
	    TokenType::GreaterEqual => (String::from("null"), String::from(">=")),
	    TokenType::Less => (String::from("null"), String::from("<")),
	    TokenType::LessEqual => (String::from("null"), String::from("<=")),
	    TokenType::LeftBrace => (String::from("null"), String::from("{")),
	    TokenType::RightBrace => (String::from("null"), String::from("}")),
	    TokenType::LeftParen => (String::from("null"), String::from("(")),
	    TokenType::RightParen => (String::from("null"), String::from(")")),
	    TokenType::Minus => (String::from("null"), String::from("-")),
	    TokenType::Plus => (String::from("null"), String::from("+")),
	    TokenType::Star => (String::from("null"), String::from("*")),
	    TokenType::Semicolon => (String::from("null"), String::from(";")),
	    TokenType::Slash => (String::from("null"), String::from("/"))
	};
	
	Self {
	    token_type: token_type,
	    lexeme: lexeme,
	    literal: literal
	}
    }
}
