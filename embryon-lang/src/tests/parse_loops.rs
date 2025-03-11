use crate::ast::*;

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
                body: Expression::Block(Block::from(
                    Expression::Loop(Box::new(Block::empty().into()))
                ))
            })],
        },
    );
}
