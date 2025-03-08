use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Const,
    Let,
    Fn,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semi,
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Identifier(Rc<str>),
    Integer(u64),
}
