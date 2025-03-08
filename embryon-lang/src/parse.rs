use crate::ast::{BinOp, Block, ConstExpression, Constant, Expression, Function, Module};
use crate::lexer::TokenStream;
use crate::tokens::Token;
use std::rc::Rc;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    ExpectedToken(Token),
    UnexpectedEoF,
}

impl Module {
    pub fn parse_body(tokens: &mut TokenStream, name: Rc<str>) -> Result<Self, ParseError> {
        let mut definitions = Vec::new();
        while let Some(token) = tokens.peek() {
            match token {
                Token::Fn => {
                    definitions.push(crate::ast::Definition::Function(Function::parse(tokens)?));
                }
                Token::Const => {
                    definitions.push(crate::ast::Definition::Constant(Constant::parse(tokens)?));
                }
                _ => {
                    dbg!(token);
                    return Err(ParseError::UnexpectedToken(token.clone()));
                }
            }
        }
        Ok(Module { name, definitions })
    }
}

impl Function {
    pub fn parse(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        tokens.expect(Token::Fn)?;
        let name = tokens.expect_identifier()?;
        tokens.expect(Token::OpenParen)?;
        tokens.expect(Token::CloseParen)?;
        let body = Expression::parse(tokens)?;
        Ok(Function {
            name,
            parameters: Vec::new(),
            body,
        })
    }
}

impl Constant {
    pub fn parse(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        tokens.expect(Token::Const)?;
        let name = tokens.expect_identifier()?;
        tokens.expect(Token::Equal)?;
        let value = ConstExpression::parse(tokens)?;
        Ok(Constant {
            name: name.into(),
            value,
        })
    }
}

impl ConstExpression {
    pub fn parse(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        match tokens.next() {
            Some(Token::Integer(value)) => Ok(ConstExpression::Integer(value)),
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::ExpectedToken(Token::Integer(0))),
        }
    }
}

impl Expression {
    pub fn parse(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        Self::parse_expression(tokens)
    }

    fn parse_expression(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        let mut expression = Self::parse_term(tokens)?;
        while let Some(token) = tokens.peek() {
            match token {
                Token::Plus => {
                    tokens.next();
                    expression = Expression::BinOp(BinOp::Add(
                        Box::new(expression),
                        Box::new(Self::parse_term(tokens)?),
                    ));
                }
                Token::Minus => {
                    tokens.next();
                    expression = Expression::BinOp(BinOp::Sub(
                        Box::new(expression),
                        Box::new(Self::parse_term(tokens)?),
                    ));
                }
                _ => break,
            }
        }
        Ok(expression)
    }

    fn parse_term(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        let mut expression = Self::parse_factor(tokens)?;
        while let Some(token) = tokens.peek() {
            match token {
                Token::Star => {
                    tokens.next();
                    expression = Expression::BinOp(BinOp::Mul(
                        Box::new(expression),
                        Box::new(Self::parse_factor(tokens)?),
                    ));
                }
                Token::Slash => {
                    tokens.next();
                    expression = Expression::BinOp(BinOp::Div(
                        Box::new(expression),
                        Box::new(Self::parse_factor(tokens)?),
                    ));
                }
                _ => break,
            }
        }
        Ok(expression)
    }

    fn parse_factor(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        match tokens.next() {
            Some(Token::OpenParen) => {
                let expression = Expression::parse(tokens)?;
                tokens.expect(Token::CloseParen)?;
                Ok(expression)
            }
            Some(Token::OpenBrace) => Self::parse_block(tokens),
            Some(Token::Identifier(name)) => {
                Ok(Expression::ConstExpression(ConstExpression::Constant(
                    Constant {
                        name,
                        value: ConstExpression::Integer(0),
                    }
                    .into(),
                )))
            }
            Some(Token::Integer(value)) => {
                Ok(Expression::ConstExpression(ConstExpression::Integer(value)))
            }
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::UnexpectedEoF),
        }
    }

    fn parse_block(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        let mut body = Vec::new();
        let mut last = None;
        while !matches!(tokens.peek(), Some(Token::CloseBrace) | Some(Token::Semi)) {
            let expr = Self::parse(tokens)?;
            match tokens.peek() {
                Some(Token::Semi) => {
                    body.push(expr);
                    tokens.next();
                }
                Some(Token::CloseBrace) => last = Some(expr),
                None => return Err(ParseError::UnexpectedEoF),
                Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
            }
        }
        tokens.expect(Token::CloseBrace)?;
        Ok(Self::Block(Block {
            body,
            last: last.map(Box::new),
        }))
    }
}
