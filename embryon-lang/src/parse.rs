use crate::ast::{
    BinOp, Block, Definition, Expression, Function, Module, Statement, Variable,
    VariableAssignment, VariableDefinition, VariableSpec,
};
use crate::lexer::TokenStream;
use crate::tokens::Token;
use std::rc::Rc;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    ExpectedToken(Token),
    UnexpectedEoF,
    ExpectedExpression,
}

impl Module {
    pub fn parse_body(tokens: &mut TokenStream, name: Rc<str>) -> Result<Self, ParseError> {
        let mut definitions = Vec::new();
        while let Some(token) = tokens.peek() {
            match token {
                Token::Fn => {
                    definitions.push(Definition::Function(Function::parse(tokens)?));
                }
                Token::Const => {
                    definitions.push(Definition::parse_constant(tokens)?);
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

impl Definition {
    pub fn parse_constant(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        tokens.expect(Token::Const)?;
        let name = tokens.expect_identifier()?;
        tokens.expect(Token::Equal)?;
        let value = Expression::parse(tokens)?;
        tokens.expect(Token::Semi)?;
        Ok(Self::Constant(Variable {
            spec: VariableSpec {
                name: name.into(),
                is_mutable: false,
            },
            value: Box::new(value),
        }))
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

impl Expression {
    pub fn parse(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        let head = tokens.peek().ok_or(ParseError::UnexpectedEoF)?.clone();
        if let Token::Identifier(name) = head {
            let next_two = (tokens.peek_ahead(1).cloned(), tokens.peek_ahead(2).cloned());
            match next_two {
                (Some(Token::Equal), Some(Token::Equal)) => (),
                (Some(Token::Equal), Some(_)) => {
                    tokens.expect_identifier()?;
                    tokens.expect(Token::Equal)?;
                    let value = Expression::parse(tokens)?;
                    return Ok(Self::VariableAssignment(VariableAssignment {
                        name: name.clone(),
                        value: Box::new(value),
                    }));
                }
                _ => (),
            }
        }
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
            Some(Token::Identifier(name)) => Ok(Expression::Variable(name)),
            Some(Token::Integer(value)) => Ok(Expression::Integer(value)),
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::UnexpectedEoF),
        }
    }

    fn parse_block(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        let mut body = Vec::new();
        let mut last = None;
        while !matches!(tokens.peek(), Some(Token::CloseBrace | Token::Semi)) {
            let expr = Statement::parse(tokens)?;
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

        let last = match last {
            None => None,
            Some(Statement::Expression(expr)) => Some(expr),
            _ => return Err(ParseError::ExpectedExpression),
        };

        Ok(Self::Block(Block {
            body,
            last: last.map(Box::new),
        }))
    }
}

impl Statement {
    pub fn parse(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        match tokens.peek() {
            Some(Token::Let) => Ok(Self::VariableDefinition(VariableDefinition::parse(tokens)?)),
            _ => Ok(Self::Expression(Expression::parse(tokens)?)),
        }
    }
}

impl VariableDefinition {
    pub fn parse(tokens: &mut TokenStream) -> Result<Self, ParseError> {
        tokens.expect(Token::Let)?;
        let is_mutable = if matches!(tokens.peek(), Some(Token::Mut)) {
            tokens.next();
            true
        } else {
            false
        };
        let identifier = tokens.expect_identifier()?;
        tokens.expect(Token::Equal)?;
        let value = Expression::parse(tokens)?;
        Ok(Self {
            spec: VariableSpec {
                name: identifier.into(),
                is_mutable,
            },
            value: Some(Box::new(value)),
        })
    }
}
