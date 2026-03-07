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
	    TokenType::STRING(val)  => {
		let lexeme = format!("\"{}\"", &val);
		(val.to_string(), lexeme)
	    }
	    ,
	    TokenType::EOF => {
		(String::from("null"), String::from(""))
	    }
	    ,
	    TokenType::BANG =>  (String::from("null"), String::from("!")),
	    TokenType::BANG_EQUAL => (String::from("null"), String::from("!=")),
	    TokenType::COMMA => (String::from("null"), String::from(",")),
	    TokenType::DOT => (String::from("null"), String::from(".")),
	    TokenType::EQUAL => (String::from("null"), String::from("=")),
	    TokenType::EQUAL_EQUAL => (String::from("null"), String::from("==")),
	    TokenType::GREATER => (String::from("null"), String::from(">")),
	    TokenType::GREATER_EQUAL => (String::from("null"), String::from(">=")),
	    TokenType::LESS => (String::from("null"), String::from("<")),
	    TokenType::LESS_EQUAL => (String::from("null"), String::from("<=")),
	    TokenType::LEFT_BRACE => (String::from("null"), String::from("{")),
	    TokenType::RIGHT_BRACE => (String::from("null"), String::from("}")),
	    TokenType::LEFT_PAREN => (String::from("null"), String::from("(")),
	    TokenType::RIGHT_PAREN => (String::from("null"), String::from(")")),
	    TokenType::MINUS => (String::from("null"), String::from("-")),
	    TokenType::PLUS => (String::from("null"), String::from("+")),
	    TokenType::STAR => (String::from("null"), String::from("*")),
	    TokenType::SEMICOLON => (String::from("null"), String::from(";")),
	    TokenType::SLASH => (String::from("null"), String::from("/")),
	    TokenType::NUMBER(num, parsed) => (format!("{:?}", &parsed), format!("{}", num)),
	    TokenType::IDENTIFIER(val) => (String::from("null"), val.to_string()),
	    TokenType::Reserved(reserved_word) => (String::from("null"), reserved_word.to_string())
	};

	Self {
	    token_type: token_type,
	    lexeme: lexeme,
	    literal: literal
	}
    }
}
