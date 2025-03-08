use crate::ast::{BinOp, Block, ConstExpression, Constant, Expression, Function, Module};
use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::module::Module as LLVMModule;
use inkwell::values::{BasicValueEnum, IntValue};

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a LLVMModule<'ctx>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    /// Compiles the specified `Function` in the given `Context` and using the specified `Builder` and `Module`.
    pub fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a LLVMModule<'ctx>,
    ) -> Self {
        Self {
            context,
            builder,
            module,
        }
    }

    pub fn compile_module(&mut self, module: &Module) -> Result<(), BuilderError> {
        println!("Compiling");
        println!("{:?}", module.definitions);
        for definition in &module.definitions {
            match definition {
                crate::ast::Definition::Function(function) => self.compile_function(function)?,
                crate::ast::Definition::Constant(constant) => self.compile_constant(constant)?,
            }
        }
        Ok(())
    }

    fn compile_function(&mut self, function: &Function) -> Result<(), BuilderError> {
        let function_type = self.context.i32_type().fn_type(&[], false);
        let func = self
            .module
            .add_function(&function.name, function_type, None);
        let entry = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry);
        let body = self.compile_expression(&function.body)?;
        self.builder.build_return(Some(&body))?;

        if func.verify(true) {
            return Ok(());
        }
        panic!("Function verification failed");
    }

    fn compile_constant(&mut self, constant: &Constant) -> Result<(), BuilderError> {
        let value = self.compile_const_expr(&constant.value)?;
        let const_type = self.context.i32_type();
        let constant = self.module.add_global(const_type, None, &constant.name);
        constant.set_constant(true);
        constant.set_initializer(&value);
        Ok(())
    }

    fn compile_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        match expression {
            Expression::ConstExpression(c) => self.compile_const_expr(c),
            Expression::BinOp(op) => self.compile_binop(op),
            Expression::Block(block) => self.compile_block(block),
        }
    }

    fn compile_const_expr(
        &mut self,
        constant: &ConstExpression,
    ) -> Result<IntValue<'ctx>, BuilderError> {
        match constant {
            ConstExpression::Integer(x) => Ok(self.context.i32_type().const_int(*x, false)),
            ConstExpression::Constant(constant) => {
                let c = self
                    .module
                    .get_global(&constant.name)
                    .unwrap_or_else(|| panic!("Constant `{}` not found", constant.name));
                self.builder
                    .build_load(self.context.i32_type(), c.as_pointer_value(), "loadconst")
                    .map(BasicValueEnum::into_int_value)
            }
        }
    }

    fn compile_binop(&mut self, op: &BinOp) -> Result<IntValue<'ctx>, BuilderError> {
        match op {
            BinOp::Add(lhs, rhs) => {
                let lhs = self.compile_expression(lhs)?;
                let rhs = self.compile_expression(rhs)?;
                self.builder.build_int_add(lhs, rhs, "addtmp")
            }
            BinOp::Sub(lhs, rhs) => {
                let lhs = self.compile_expression(lhs)?;
                let rhs = self.compile_expression(rhs)?;
                self.builder.build_int_sub(lhs, rhs, "subtmp")
            }
            BinOp::Mul(lhs, rhs) => {
                let lhs = self.compile_expression(lhs)?;
                let rhs = self.compile_expression(rhs)?;
                self.builder.build_int_mul(lhs, rhs, "multmp")
            }
            BinOp::Div(lhs, rhs) => {
                let lhs = self.compile_expression(lhs)?;
                let rhs = self.compile_expression(rhs)?;
                self.builder.build_int_signed_div(lhs, rhs, "divtmp")
            }
        }
    }

    fn compile_block(&mut self, block: &Block) -> Result<IntValue<'ctx>, BuilderError> {
        if let Some(err) = block
            .body
            .iter()
            .map(|expr| self.compile_expression(expr))
            .filter_map(Result::err)
            .next()
        {
            return Err(err);
        }
        if let Some(last) = &block.last {
            self.compile_expression(last)
        } else {
            Ok(self.context.i32_type().const_int(0, false))
        }
    }
}
