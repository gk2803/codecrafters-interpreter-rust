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
		Some('(') => self.add_token(Token::new(TokenType::LeftParen)),
		Some(')') => self.add_token(Token::new(TokenType::RightParen)),
		Some('{') => self.add_token(Token::new(TokenType::LeftBrace)),
		Some('}') => self.add_token(Token::new(TokenType::RightBrace)),
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
		    self.add_token(Token::new(TokenType::Less))
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

		Some(c @ '"') =>
		{
		    let mut val = String::new();
		    self.consume_while(|c| c != '"', &mut val);
		    if Some('"') != self.peek() {
			is_err = true;
			eprintln!("[line {}] Error: Unterminated string.", self.line)
		    } else {
			self.advance();
			self.add_token(Token::new(TokenType::String(val)));
		    }
		    
		},
		
		Some(c) if c.is_numeric() => { 
		    let mut num = c.to_string();
		    self.consume_while(|c| c.is_numeric() || c == '.', &mut num);
		    let parsed: f64 = num.parse().unwrap();
		    self.add_token(Token::new(TokenType::Number(num, parsed)));
		},
		Some(c) => 
		{
		    is_err = true;
		    eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
		    
		}
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
