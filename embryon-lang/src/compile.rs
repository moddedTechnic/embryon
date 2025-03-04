use crate::ast::{Function, Module};
use crate::llvm;

impl Module {
    pub fn compile(&self, context: &llvm::Context, module: &llvm::Module) {
        for definition in &self.definitions {
            match definition {
                crate::ast::Definition::Function(function) => {
                    function.compile(context, module);
                }
                crate::ast::Definition::Constant(_constant) => {
                    todo!("compile constant");
                }
            }
        }
    }
}

impl Function {
    pub fn compile(&self, context: &llvm::Context, module: &llvm::Module) {
        let builder = context.builder();
        let function_type = llvm::FunctionType::new(llvm::Type::Int32, Vec::new());
        let function = module.add_function(&self.name, function_type);
        let entry = function.append_basic_block("entry");
        builder.position_at_end(entry);
        builder.return_(llvm::Value::ConstU32(0));
    }
}
