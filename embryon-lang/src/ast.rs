use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: Rc<str>,
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone)]
pub enum Definition {
    Function(Function),
    Constant(Constant),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Constant>,
    pub body: Expression,
}

#[derive(Debug, Clone)]
pub struct Constant {
    pub name: Rc<str>,
    pub value: ConstExpression,
}

#[derive(Debug, Clone)]
pub enum ConstExpression {
    Integer(u64),
    Constant(Box<Constant>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    ConstExpression(ConstExpression),
    BinOp(BinOp),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}
