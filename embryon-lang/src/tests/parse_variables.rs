use crate::ast::*;

#[test]
fn let_immutable() {
    let source = "fn main() { let x = 1; }";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "let_immutable".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "let_immutable".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Expression::Block(Block {
                    body: vec![Statement::VariableDefinition(VariableDefinition {
                        spec: VariableSpec {
                            name: "x".into(),
                            is_mutable: false,
                        },
                        value: Some(Box::new(Expression::Integer(1)))
                    })],
                    last: None
                })
            })],
        },
    );
}

#[test]
fn let_mutable() {
    let source = "fn main() { let mut x = 1; }";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "let_mutable".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "let_mutable".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Expression::Block(Block {
                    body: vec![Statement::VariableDefinition(VariableDefinition {
                        spec: VariableSpec {
                            name: "x".into(),
                            is_mutable: true,
                        },
                        value: Some(Box::new(Expression::Integer(1)))
                    })],
                    last: None
                })
            })],
        },
    );
}

#[test]
fn variable_access() {
    let source = "fn main() { x }";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "variable_access".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "variable_access".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Expression::Block(Block {
                    body: vec![],
                    last: Some(Box::new(Expression::Variable("x".into())))
                })
            })],
        },
    );
}

#[test]
fn variable_assign() {
    let source = "fn main() { x = 2 }";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "variable_assign".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "variable_assign".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Expression::Block(Block {
                    body: vec![],
                    last: Some(Box::new(Expression::VariableAssignment(
                        VariableAssignment {
                            name: "x".into(),
                            value: Box::new(Expression::Integer(2))
                        }
                    )))
                })
            })],
        },
    );
}