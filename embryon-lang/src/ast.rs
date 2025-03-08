use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: Rc<str>,
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Definition {
    Function(Function),
    Constant(Variable),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<VariableSpec>,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableSpec {
    pub name: Rc<str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub spec: VariableSpec,
    pub value: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Integer(u64),
    Variable(Rc<str>),
    BinOp(BinOp),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub body: Vec<Statement>,
    pub last: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Expression(Expression),
    VariableDefinition(VariableDefinition),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDefinition {
    pub name: Rc<str>,
    pub is_mutable: bool,
    pub value: Option<Box<Expression>>,
}
