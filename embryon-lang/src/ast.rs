use std::rc::Rc;

macro_rules! impl_from {
    ($type:ty | $src:ty => $variant:ident) => {
        impl From<$src> for $type {
            fn from(value: $src) -> Self {
                Self::$variant(value)
            }
        }
    };
    ($type:ty | $src:ident) => {
        impl_from!($type | $src => $src);
    };
    ($type:ty | $($src:ident => $variant:ident),+) => {
        $(impl_from!($type | $src => $variant);)+
    };
    ($type:ty | $src:ident, $($rest:tt),+) => {
        impl_from!($type | $src);
        impl_from!($type | $($rest),+);
    };
}

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

impl_from!(Definition | Variable => Constant);
impl_from!(Definition | Function);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<VariableSpec>,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableSpec {
    pub name: Rc<str>,
    pub is_mutable: bool,
}

impl VariableSpec {
    pub fn new(name: impl Into<Rc<str>>) -> Self {
        Self {
            name: name.into(),
            is_mutable: false,
        }
    }

    pub fn mutable(self) -> Self {
        Self {
            is_mutable: true,
            ..self
        }
    }
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
    VariableAssignment(VariableAssignment),
    Loop(Box<Expression>),
    Break, Continue,
}

impl_from!(Expression | u64 => Integer);
impl_from!(Expression | BinOp, Block, VariableAssignment);

impl From<Expression> for Option<Box<Expression>> {
    fn from(expr: Expression) -> Self {
        Some(Box::new(expr))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Block {
    pub body: Vec<Statement>,
    pub last: Option<Box<Expression>>,
}

impl Block {
    pub fn empty() -> Self {
        Self::default()
    }
}

impl From<Expression> for Block {
    fn from(expr: Expression) -> Self {
        Self {
            body: vec![],
            last: Some(Box::new(expr)),
        }
    }
}

impl FromIterator<Statement> for Block {
    fn from_iter<T: IntoIterator<Item = Statement>>(iter: T) -> Self {
        Self {
            body: iter.into_iter().collect(),
            last: None,
        }
    }
}

impl<T> From<T> for Block
where
    T: IntoIterator<Item = Statement>,
{
    fn from(iter: T) -> Self {
        Self::from_iter(iter)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Expression(Expression),
    VariableDefinition(VariableDefinition),
}

impl_from!(Statement | Expression, VariableDefinition);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDefinition {
    pub spec: VariableSpec,
    pub value: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableAssignment {
    pub name: Rc<str>,
    pub value: Box<Expression>,
}
