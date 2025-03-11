use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Const,
    Let,
    Mut,
    Fn,
    Loop,
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
