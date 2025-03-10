use crate::ast::*;

#[test]
fn simple_function() {
    let source = "fn main() 0";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let _program = Module::parse_body(&mut tokens, "simple_function".into()).unwrap();
}

#[test]
fn simple_constant() {
    let source = "const x = 0;";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "simple_constant".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "simple_constant".into(),
            definitions: vec![Definition::Constant(Variable {
                spec: VariableSpec {
                    name: "x".into(),
                    is_mutable: false
                },
                value: Box::new(Expression::Integer(0)),
            })],
        },
    );
}