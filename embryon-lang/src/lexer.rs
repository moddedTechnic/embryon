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

    fn peek_char(&self, lookahead: usize) -> Option<char> {
        self.source
            .as_bytes()
            .get(self.cursor + lookahead)
            .copied()
            .map(|c| c as char)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char(0) {
            if !c.is_whitespace() {
                break;
            }
            self.cursor += 1;
        }
    }

    fn skip_until(&mut self, sequence: &str) {
        while self.cursor < self.source.len() && !self.source[self.cursor..].starts_with(sequence) {
            self.cursor += 1;
        }
    }

    fn skip_comment(&mut self) {
        self.skip_whitespace();
        if self.peek_char(0) != Some('/') {
            return
        }
        if self.peek_char(1) == Some('/') {
            self.skip_until("\n");
            self.skip_whitespace();
            self.skip_comment();
            return
        }
        if self.peek_char(1) == Some('*') {
            self.skip_until("*/");
            self.cursor += 2;
            self.skip_whitespace();
            self.skip_comment();
        }
    }

    fn read_next(&mut self) {
        if self.cursor >= self.source.len() {
            self.head = None;
            return;
        }
        self.skip_comment();
        let c = self.peek_char(0);
        let c = if let Some(c) = c {
            c
        } else {
            self.head = None;
            return;
        };
        if let Some(token) = Self::from_symbol(c) {
            self.cursor += 1;
            self.head = Some(token);
        } else {
            self.head = match c {
                '0'..='9' => self.read_number(),
                _ => self.read_identifier(),
            }
        };
    }

    fn from_symbol(symbol: char) -> Option<Token> {
        match symbol {
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            '{' => Some(Token::OpenBrace),
            '}' => Some(Token::CloseBrace),
            ';' => Some(Token::Semi),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '/' => Some(Token::Slash),
            '=' => Some(Token::Equal),
            _ => None,
        }
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
            "mut" => Some(Token::Mut),
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

#[cfg(test)]
mod tests {
    use crate::lexer::TokenStream;
    use crate::tokens::Token;

    #[test]
    fn lex_symbols() {
        let source = "(){}+-*/=;";
        let mut lex = TokenStream::new(source.into());
        assert_eq!(lex.next(), Some(Token::OpenParen));
        assert_eq!(lex.next(), Some(Token::CloseParen));
        assert_eq!(lex.next(), Some(Token::OpenBrace));
        assert_eq!(lex.next(), Some(Token::CloseBrace));
        assert_eq!(lex.next(), Some(Token::Plus));
        assert_eq!(lex.next(), Some(Token::Minus));
        assert_eq!(lex.next(), Some(Token::Star));
        assert_eq!(lex.next(), Some(Token::Slash));
        assert_eq!(lex.next(), Some(Token::Equal));
        assert_eq!(lex.next(), Some(Token::Semi));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lex_identifiers() {
        let source = "foobar";
        let mut lex = TokenStream::new(source.into());
        assert_eq!(lex.next(), Some(Token::Identifier("foobar".into())));
    }

    #[test]
    fn lex_numbers() {
        let source = "123";
        let mut lex = TokenStream::new(source.into());
        assert_eq!(lex.next(), Some(Token::Integer(123)));
    }

    #[test]
    fn lex_keywords() {
        let source = "const let mut fn";
        let mut lex = TokenStream::new(source.into());
        assert_eq!(lex.next(), Some(Token::Const));
        assert_eq!(lex.next(), Some(Token::Let));
        assert_eq!(lex.next(), Some(Token::Mut));
        assert_eq!(lex.next(), Some(Token::Fn));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lex_single_comment() {
        let source = "// this is a comment";
        let mut lex = TokenStream::new(source.into());
        assert_eq!(lex.next(), None);

        let source = "// this is a comment\n// this is another comment";
        let mut lex = TokenStream::new(source.into());
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lex_multi_comment() {
        let source = "/* this is a comment */";
        let mut lex = TokenStream::new(source.into());
        assert_eq!(lex.next(), None);

        let source = "/* this is \n a comment *//* this is another comment */";
        let mut lex = TokenStream::new(source.into());
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lex_mixed_comments() {
        let source = "/* this is a comment */ // this is another comment";
        let mut lex = TokenStream::new(source.into());
        assert_eq!(lex.next(), None);
    }
}
