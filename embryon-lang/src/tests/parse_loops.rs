use crate::ast::*;
use crate::utils::IntoExpression;

#[test]
fn empty_loop() {
    let source = "fn main() { loop {} }";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "empty_loop".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "empty_loop".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Block::from(Expression::Loop(Box::new(Block::empty().into()))).into()
            })],
        },
    );
}

#[test]
fn simple_expression_loop() {
    let source = "fn main() { loop 5; }";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "simple_expression_loop".into()).unwrap();
    assert_eq!(tokens.next(), None);

    assert_eq!(
        program,
        Module {
            name: "simple_expression_loop".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Block::from(Expression::Loop(Box::new(Expression::Integer(5)))).into()
            })],
        },
    );
}

#[test]
fn complex_block_loop() {
    let source = "fn main() { loop { let x = 1; x + 2 } }";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "complex_block_loop".into()).unwrap();
    assert_eq!(tokens.next(), None);

    assert_eq!(
        program,
        Module {
            name: "complex_block_loop".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Block::from(Expression::Loop(Box::new(
                    Block {
                        body: vec![
                            VariableDefinition {
                                spec: VariableSpec::new("x"),
                                value: Expression::Integer(1).into(),
                            }
                            .into()
                        ],
                        last: BinOp::Add(
                            Box::new(Expression::Variable("x".into())),
                            Box::new(Expression::Integer(2)),
                        )
                        .into_expression()
                        .into()
                    }
                    .into()
                )))
                .into(),
            })],
        },
    );
}
