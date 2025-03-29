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
pub mod utils;

#[cfg(test)]
mod tests;

pub fn lex(source: &str) -> lexer::TokenStream {
    lexer::TokenStream::new(source.into())
}

pub fn parse(mut tokens: lexer::TokenStream) -> Result<ast::Module, parse::ParseError> {
    ast::Module::parse_body(&mut tokens, "main".into())
}

pub fn compile(program: &ast::Module, path: &Path) {
    // TODO: we need an optimisation pass on the AST which removes unnecessary allocations,
    //       as infinite loops with a stack allocation in the body currently cause a stack overflow
    let context = Context::create();
    let module = context.create_module(path.file_stem().unwrap().to_str().unwrap());
    module.set_source_file_name(path.to_str().unwrap());
    let builder = context.create_builder();

    let mut compiler = Compiler::new(&context, &builder, &module);
    compiler.compile_module(program).unwrap();

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
