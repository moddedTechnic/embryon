use std::rc::Rc;

pub struct Module {
    pub name: Rc<str>,
    pub definitions: Vec<Definition>,
}

pub enum Definition {
    Function(Function),
    Constant(Constant),
}

pub struct Function {
    pub name: String,
    pub parameters: Vec<Constant>,
    pub body: Expression,
}

pub struct Constant {
    pub name: Rc<str>,
    pub value: ConstExpression,
}

pub enum ConstExpression {
    Integer(i64),
    Constant(Box<Constant>),
}

pub enum Expression {
    ConstExpression(ConstExpression),
    BinOp(BinOp),
}

pub enum BinOp {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}
