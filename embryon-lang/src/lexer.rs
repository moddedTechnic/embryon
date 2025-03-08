use crate::parse::ParseError;
use crate::tokens::Token;
use std::rc::Rc;

pub struct TokenStream {
    source: Rc<str>,
    head: Option<Token>,
    cursor: usize,
}

impl TokenStream {
    pub fn new(source: Rc<str>) -> Self {
        Self {
            source,
            head: None,
            cursor: 0,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.head.is_none() {
            self.read_next();
        }
        self.head.as_ref()
    }

    pub fn expect(&mut self, token: Token) -> Result<Token, ParseError> {
        match self.next() {
            Some(t) if t == token => Ok(token),
            Some(t) => Err(ParseError::UnexpectedToken(t)),
            None => Err(ParseError::UnexpectedEoF),
        }
    }

    pub fn expect_identifier(&mut self) -> Result<String, ParseError> {
        match self.next() {
            Some(Token::Identifier(name)) => Ok(name.to_string()),
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::UnexpectedEoF),
        }
    }

    fn read_next(&mut self) {
        if self.cursor >= self.source.len() {
            self.head = None;
            return;
        }
        let c = self.source.as_bytes().get(self.cursor);
        let c = if let Some(c) = c {
            *c as char
        } else {
            self.head = None;
            return;
        };
        self.head = match c {
            _ if c.is_whitespace() => {
                self.cursor += 1;
                return self.read_next();
            }
            '(' => {
                self.cursor += 1;
                Some(Token::OpenParen)
            }
            ')' => {
                self.cursor += 1;
                Some(Token::CloseParen)
            }
            '+' => {
                self.cursor += 1;
                Some(Token::Plus)
            }
            '-' => {
                self.cursor += 1;
                Some(Token::Minus)
            }
            '*' => {
                self.cursor += 1;
                Some(Token::Star)
            }
            '/' => {
                self.cursor += 1;
                Some(Token::Slash)
            }
            '=' => {
                self.cursor += 1;
                Some(Token::Equal)
            }
            '0'..='9' => self.read_number(),
            _ => self.read_identifier(),
        };
    }

    fn read_number(&mut self) -> Option<Token> {
        let start = self.cursor;
        while let Some(c) = self.source.get(self.cursor..self.cursor + 1) {
            if c.chars().all(|c| c.is_ascii_digit()) {
                self.cursor += 1;
            } else {
                break;
            }
        }
        let value = self
            .source
            .get(start..self.cursor)
            .unwrap()
            .parse()
            .unwrap();
        Some(Token::Integer(value))
    }

    fn read_identifier(&mut self) -> Option<Token> {
        let start = self.cursor;
        while let Some(c) = self.source.get(self.cursor..self.cursor + 1) {
            if c.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                self.cursor += 1;
            } else {
                break;
            }
        }
        let name = self.source.get(start..self.cursor).unwrap().to_string();
        match name.as_str() {
            "const" => Some(Token::Const),
            "let" => Some(Token::Let),
            "fn" => Some(Token::Fn),
            _ if !name.is_empty() => Some(Token::Identifier(Rc::from(name))),
            _ => None,
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_none() {
            self.read_next();
        }
        self.head.take()
    }
}
