use crate::ast::Expression;

pub trait IntoExpression {
    fn into_expression(self) -> Expression;
}

impl<T> IntoExpression for T where Expression: From<T> {
    fn into_expression(self) -> Expression {
        Expression::from(self)
    }
}
