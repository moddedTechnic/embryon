use std::path::Path;

pub mod ast;
pub mod lexer;
pub mod llvm;
pub mod parse;
pub mod tokens;
mod compile;

pub fn lex(source: &str) -> lexer::TokenStream {
    lexer::TokenStream::new(source.into())
}

pub fn parse(mut tokens: lexer::TokenStream) -> Result<ast::Module, parse::ParseError> {
    ast::Module::parse_body(&mut tokens, "main".into())
}

pub fn compile(program: ast::Module, path: &Path) {
    let context = llvm::Context::new();
    let module = context.module(path.file_stem().unwrap().to_str().unwrap());
    program.compile(&context, &module);
    module.set_target("thumbv7em-none-eabi");
    module.write_bitcode(path.with_extension("bc").to_str().unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_function() {
        let source = "fn main() 0";
        let mut tokens = crate::lexer::TokenStream::new(source.into());
        let _program = crate::ast::Module::parse_body(&mut tokens, "simple_function".into()).unwrap();
    }

    #[test]
    fn simple_constant() {
        let source = "const x = 0";
        let mut tokens = crate::lexer::TokenStream::new(source.into());
        let _program = crate::ast::Module::parse_body(&mut tokens, "simple_constant".into()).unwrap();
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
