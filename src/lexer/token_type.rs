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
    IDENTIFIER(String),

    // Reserved Words
    Reserved(ReservedWord)
}



#[derive(Debug)]
pub enum ReservedWord {
    AND, CLASS, ELSE, FALSE, FOR, FUN, IF,
    NIL, OR, PRINT, RETURN, SUPER, THIS,
    TRUE, VAR, WHILE
}

impl Display for ReservedWord {
    fn fmt(&self, f:&mut Formatter<'_>) -> Result<(), Error> {
	let s = format!("{:?}", self).to_lowercase();
	write!(f, "{}", s)
    }
}

pub fn reserved_word_from_string(s: &str) -> Option<ReservedWord> {
    match s {
        "and" => Some(ReservedWord::AND),
        "class" => Some(ReservedWord::CLASS),
        "else" => Some(ReservedWord::ELSE),
        "false" => Some(ReservedWord::FALSE),
        "for" => Some(ReservedWord::FOR),
        "fun" => Some(ReservedWord::FUN),
        "if" => Some(ReservedWord::IF),
        "nil" => Some(ReservedWord::NIL),
        "or" => Some(ReservedWord::OR),
        "print" => Some(ReservedWord::PRINT),
        "return" => Some(ReservedWord::RETURN),
        "super" => Some(ReservedWord::SUPER),
        "this" => Some(ReservedWord::THIS),
        "true" => Some(ReservedWord::TRUE),
        "var" => Some(ReservedWord::VAR),
        "while" => Some(ReservedWord::WHILE),
        _ => None,
    }
}


impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
	match self {
	    TokenType::STRING(_) => write!(f, "STRING"),
	    TokenType::NUMBER(_,_) => write!(f, "NUMBER"),
	    TokenType::IDENTIFIER(_) => write!(f, "IDENTIFIER"),
	    TokenType::Reserved(word) => write!(f, "{:?}", word),
	    _ => write!(f, "{:?}", self)
	}
	
    }
}
