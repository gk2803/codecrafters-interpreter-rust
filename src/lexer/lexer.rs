use super::token::Token;
use super::token_type::TokenType;

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
	println!("{}", token);
	self.tokens.push(token);
    }

    pub fn consume_while<F: Fn(char) -> bool> (
	&mut self,
	until: F,
	buffer: &mut String
    ) {
	while let Some(c) = self.peek() {
	    if !until(c) {
		break;
	    }
	    buffer.push(c);
	    self.advance();
	}
    }



    pub fn tokenize(&mut self) {

	let mut is_err = false;

	loop {
	    match self.advance() {
		Some('(') => self.add_token(Token::new(TokenType::LEFT_PAREN)),
		Some(')') => self.add_token(Token::new(TokenType::RIGHT_PAREN)),
		Some('{') => self.add_token(Token::new(TokenType::LEFT_BRACE)),
		Some('}') => self.add_token(Token::new(TokenType::RIGHT_BRACE)),
		Some('+') => self.add_token(Token::new(TokenType::PLUS)),
		Some('*') => self.add_token(Token::new(TokenType::STAR)),
		Some(',') => self.add_token(Token::new(TokenType::COMMA)),
		Some('.') => self.add_token(Token::new(TokenType::DOT)),
		Some(';') => self.add_token(Token::new(TokenType::SEMICOLON)),
		Some('-') => self.add_token(Token::new(TokenType::MINUS)),
		Some('=') => if let Some('=') = self.peek() {
		    self.advance();
		    self.add_token(Token::new(TokenType::EQUAL_EQUAL));
		} else {
		    self.add_token(Token::new(TokenType::EQUAL));
		}
		,
		Some('!') => if let Some('=') = self.peek() {
		    self.advance();
		    self.add_token(Token::new(TokenType::BANG_EQUAL));
		} else {
		    self.add_token(Token::new(TokenType::BANG));
		},
		Some('<') => if let Some('=') = self.peek() {
		    self.advance();
		    self.add_token(Token::new(TokenType::LESS_EQUAL));
		} else {
		    self.add_token(Token::new(TokenType::LESS))
		},
		Some('>') => if let Some('=') = self.peek() {
		    self.advance();
		    self.add_token(Token::new(TokenType::GREATER_EQUAL));
		} else {
		    self.add_token(Token::new(TokenType::GREATER));
		},
		Some('/') => if let Some('/') = self.peek() {
		    while self.peek() != Some('\n') && !self.is_at_end() {
			self.advance();
		    }
		    continue;
		} else {
		    self.add_token(Token::new(TokenType::SLASH));
		},
		Some(' ') | Some('\t') | Some('\n') =>
		    continue,

		Some(c @ '"') =>
		{
		    let mut val = String::new();
		    self.consume_while(|c| c != '"', &mut val);
		    if Some('"') != self.peek() {
			is_err = true;
			eprintln!("[line {}] Error: Unterminated string.", self.line)
		    } else {
			self.advance();
			self.add_token(Token::new(TokenType::STRING(val)));
		    }
		    
		},
		
		Some(c) if c.is_numeric() => { 
		    let mut num = c.to_string();
		    self.consume_while(|c| c.is_numeric() || c == '.', &mut num);
		    let parsed: f64 = num.parse().unwrap();
		    self.add_token(Token::new(TokenType::NUMBER(num, parsed)));
		},
		Some(c) if c.is_alphabetic() ||c == '_' => {
		    let mut val = c.to_string();
		    self.consume_while(|c| c.is_alphanumeric(), &mut val);
		    self.add_token(Token::new(TokenType::IDENTIFIER(val)));
		}
		,
		Some(c) => 
		{
		    is_err = true;
		    eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
		    
		},
		None => {
		    self.add_token(Token::new(TokenType::EOF));
		    break;
		},

	    }
	    
	}

	if is_err {
	    std::process::exit(65);
	}

    }
}
