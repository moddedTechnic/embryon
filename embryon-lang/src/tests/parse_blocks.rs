use crate::ast::*;

#[test]
fn fn_empty_block() {
    let source = "fn main() {}";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "block".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "block".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Expression::Block(Block::empty())
            })],
        },
    );
}

#[test]
fn fn_simple_block() {
    let source = "fn main() { 0 }";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "simple_block".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "simple_block".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Block::from(Expression::Integer(0)).into(),
            })],
        },
    );
}

#[test]
fn fn_compound_block() {
    let source = "fn main() { 1; 2 }";
    let mut tokens = crate::lexer::TokenStream::new(source.into());
    let program = Module::parse_body(&mut tokens, "compound_block".into()).unwrap();

    assert_eq!(
        program,
        Module {
            name: "compound_block".into(),
            definitions: vec![Definition::Function(Function {
                name: "main".into(),
                parameters: vec![],
                body: Expression::Block(Block {
                    body: vec![Expression::Integer(1).into()],
                    last: Some(Box::new(Expression::Integer(2)))
                })
            })],
        },
    );
}
