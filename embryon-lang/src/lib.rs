use crate::compile::Compiler;
use inkwell::context::Context;
// use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple};
// use inkwell::OptimizationLevel;
use std::path::Path;

pub mod ast;
mod compile;
pub mod lexer;
pub mod parse;
pub mod tokens;

pub fn lex(source: &str) -> lexer::TokenStream {
    lexer::TokenStream::new(source.into())
}

pub fn parse(mut tokens: lexer::TokenStream) -> Result<ast::Module, parse::ParseError> {
    ast::Module::parse_body(&mut tokens, "main".into())
}

pub fn compile(program: ast::Module, path: &Path) {
    let context = Context::create();
    let module = context.create_module(path.file_stem().unwrap().to_str().unwrap());
    let builder = context.create_builder();

    let mut compiler = Compiler::new(&context, &builder, &module);
    compiler.compile_module(&program).unwrap();

    // Write ll to file
    module.print_to_file(path.with_extension("ll")).unwrap();

    // Target::initialize_arm(&InitializationConfig::default());
    // let target = Target::from_triple(&TargetTriple::create("thumbv7em-none-eabi")).unwrap();
    // let target_machine = target
    //     .create_target_machine(
    //         &TargetMachine::get_default_triple(),
    //         "cortex-m4",
    //         "",
    //         OptimizationLevel::Default,
    //         RelocMode::Default,
    //         CodeModel::Default,
    //     )
    //     .unwrap();
    //
    // target_machine
    //     .write_to_file(&module, FileType::Assembly, &path.with_extension("s"))
    //     .unwrap();
}

#[cfg(test)]
mod tests {
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
        )
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
        )
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
        )
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
        )
    }

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
                    body: Expression::Block(Block {
                        body: vec![],
                        last: None
                    })
                })],
            },
        )
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
                    body: Expression::Block(Block {
                        body: vec![],
                        last: Some(Box::new(Expression::Integer(0)))
                    })
                })],
            },
        )
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
                        body: vec![Statement::Expression(Expression::Integer(1))],
                        last: Some(Box::new(Expression::Integer(2)))
                    })
                })],
            },
        )
    }

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
        )
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
        )
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
        )
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
                        last: Some(Box::new(Expression::VariableAssignment(VariableAssignment {
                            name: "x".into(),
                            value: Box::new(Expression::Integer(2))
                        })))
                    })
                })],
            },
        )
    }
}
