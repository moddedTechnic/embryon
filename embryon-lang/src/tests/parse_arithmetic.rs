use crate::ast::*;

#[test]
fn fn_add() {
    let source = "fn main() 1 + 2";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "add".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "add".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Expression::BinOp(BinOp::Add(
                    Box::new(Expression::Integer(1)),
                    Box::new(Expression::Integer(2)),
                ))
            })],
        },
    );
}

#[test]
fn fn_sub() {
    let source = "fn main() 1 - 2";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "sub".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "sub".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Expression::BinOp(BinOp::Sub(
                    Box::new(Expression::Integer(1)),
                    Box::new(Expression::Integer(2)),
                ))
            })],
        },
    );
}

#[test]
fn fn_mul() {
    let source = "fn main() 1 * 2";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "mul".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "mul".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Expression::BinOp(BinOp::Mul(
                    Box::new(Expression::Integer(1)),
                    Box::new(Expression::Integer(2)),
                ))
            })],
        },
    );
}

#[test]
fn fn_div() {
    let source = "fn main() 1 / 2";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "div".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "div".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Expression::BinOp(BinOp::Div(
                    Box::new(Expression::Integer(1)),
                    Box::new(Expression::Integer(2)),
                ))
            })],
        },
    );
}
