use crate::compile::Compiler;
use inkwell::context::Context;
use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple};
use inkwell::OptimizationLevel;
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

    Target::initialize_arm(&InitializationConfig::default());
    let target = Target::from_triple(&TargetTriple::create("thumbv7em-none-eabi")).unwrap();
    let target_machine = target
        .create_target_machine(
            &TargetMachine::get_default_triple(),
            "cortex-m4",
            "",
            OptimizationLevel::Default,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    target_machine
        .write_to_file(&module, FileType::Assembly, &path.with_extension("s"))
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_function() {
        let source = "fn main() 0";
        let mut tokens = crate::lexer::TokenStream::new(source.into());
        let _program =
            crate::ast::Module::parse_body(&mut tokens, "simple_function".into()).unwrap();
    }

    #[test]
    fn simple_constant() {
        let source = "const x = 0";
        let mut tokens = crate::lexer::TokenStream::new(source.into());
        let _program =
            crate::ast::Module::parse_body(&mut tokens, "simple_constant".into()).unwrap();
    }

    #[test]
    fn fn_add() {
        let source = "fn main() 1 + 2";
        let mut tokens = crate::lexer::TokenStream::new(source.into());
        let _program = crate::ast::Module::parse_body(&mut tokens, "add".into()).unwrap();
    }

    #[test]
    fn fn_sub() {
        let source = "fn main() 1 - 2";
        let mut tokens = crate::lexer::TokenStream::new(source.into());
        let _program = crate::ast::Module::parse_body(&mut tokens, "sub".into()).unwrap();
    }

    #[test]
    fn fn_mul() {
        let source = "fn main() 1 * 2";
        let mut tokens = crate::lexer::TokenStream::new(source.into());
        let _program = crate::ast::Module::parse_body(&mut tokens, "mul".into()).unwrap();
    }

    #[test]
    fn fn_div() {
        let source = "fn main() 1 / 2";
        let mut tokens = crate::lexer::TokenStream::new(source.into());
        let _program = crate::ast::Module::parse_body(&mut tokens, "div".into()).unwrap();
    }
}
