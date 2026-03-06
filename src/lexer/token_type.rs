use std::fmt::{Formatter, Display, Error};

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals.
    STRING(String), NUMBER(String, f64),

    // Keywords.
    EOF,

    // Identifier,
    IDENTIFIER(String)
}





impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
	match self {
	    TokenType::STRING(_) => write!(f, "STRING"),
	    TokenType::NUMBER(_,_) => write!(f, "NUMBER"),
	    TokenType::IDENTIFIER(_) => write!(f, "IDENTIFIER"),
	    _ => write!(f, "{:?}", self)
	}
	
    }
}
