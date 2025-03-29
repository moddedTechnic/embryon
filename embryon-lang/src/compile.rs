use crate::ast::{
    BinOp, Block, Expression, Function, Module, Statement, Variable, VariableAssignment,
    VariableDefinition, VariableSpec,
};
use inkwell::basic_block::BasicBlock;
use inkwell::builder::{Builder, BuilderError};
use inkwell::context::Context;
use inkwell::module::Module as LLVMModule;
use inkwell::values::{BasicValueEnum, FunctionValue, IntValue, PointerValue};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
enum NamedValue<'ctx> {
    Constant,
    Variable(VariableSpec, PointerValue<'ctx>),
}

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a LLVMModule<'ctx>,

    current_function: Option<FunctionValue<'ctx>>,
    loop_headers: Vec<BasicBlock<'ctx>>,
    loop_exits: Vec<BasicBlock<'ctx>>,
    named_values: HashMap<Rc<str>, NamedValue<'ctx>>,
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
            current_function: None,
            loop_headers: Vec::new(),
            loop_exits: Vec::new(),
            named_values: HashMap::new(),
        }
    }

    pub fn compile_module(&mut self, module: &Module) -> Result<(), BuilderError> {
        println!("Compiling");
        println!("{:#?}", module.definitions);
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
        self.current_function = Some(func);
        let body = self.compile_expression(&function.body)?;
        if let Some(body) = body {
            self.builder.build_return(Some(&body))?;
        } else {
            // TODO: when actual function types are implemented, this case should return void (unit)
            self.builder
                .build_return(Some(&self.context.i32_type().const_int(0, false)))?;
        }
        self.current_function = None;

        if func.verify(true) {
            return Ok(());
        }
        println!("{func:#?}");
        panic!("Function verification failed");
    }

    fn compile_constant(&mut self, constant: &Variable) -> Result<(), BuilderError> {
        let name = constant.spec.name.clone();
        let value = self.compile_expression(&constant.value)?;
        let const_type = self.context.i32_type();
        let constant = self
            .module
            .add_global(const_type, None, &constant.spec.name);
        constant.set_constant(true);
        if let Some(value) = value {
            constant.set_initializer(&value);
        }
        self.named_values.insert(name, NamedValue::Constant);
        Ok(())
    }

    fn compile_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<Option<IntValue<'ctx>>, BuilderError> {
        match expression {
            Expression::Integer(x) => Ok(Some(self.context.i32_type().const_int(*x, false))),
            Expression::Variable(name) => self.compile_variable_access(name),
            Expression::VariableAssignment(assign) => self.compile_variable_assign(assign),
            Expression::BinOp(op) => self.compile_binop(op),
            Expression::Block(block) => self.compile_block(block),
            Expression::Loop(body) => self.compile_loop(None, None, None, body),
            f @ Expression::Break | f @ Expression::Continue => self.compile_loop_flow(f),
        }
    }

    fn compile_variable_access(
        &mut self,
        name: &Rc<str>,
    ) -> Result<Option<IntValue<'ctx>>, BuilderError> {
        match self.named_values.get(name) {
            Some(NamedValue::Constant) => {
                let c = self
                    .module
                    .get_global(name)
                    .unwrap_or_else(|| panic!("Constant {name} not found"));
                self.builder
                    .build_load(self.context.i32_type(), c.as_pointer_value(), "loadconst")
                    .map(BasicValueEnum::into_int_value)
                    .map(Some)
            }
            Some(NamedValue::Variable(_, ptr)) => self
                .builder
                .build_load(self.context.i32_type(), *ptr, &format!("load_{name}"))
                .map(BasicValueEnum::into_int_value)
                .map(Some),
            None => panic!("Variable {name} not found"),
        }
    }

    fn compile_variable_assign(
        &mut self,
        assign: &VariableAssignment,
    ) -> Result<Option<IntValue<'ctx>>, BuilderError> {
        let named_value = self.named_values.get(&assign.name).cloned();
        match named_value {
            Some(NamedValue::Constant) => panic!("Can't reassign to constant"),
            Some(NamedValue::Variable(spec, ptr)) => {
                assert!(spec.is_mutable, "Variable {} is not mutable", spec.name);
                let value = self.compile_expression(&assign.value)?;
                if let Some(value) = value {
                    self.builder.build_store(ptr, value)?;
                }
                Ok(value)
            }
            None => panic!("Variable {} not found", assign.name),
        }
    }

    fn compile_binop(&mut self, op: &BinOp) -> Result<Option<IntValue<'ctx>>, BuilderError> {
        match op {
            BinOp::Add(lhs, rhs) => {
                let lhs = self
                    .compile_expression(lhs)?
                    .expect("Cannot add unit to number");
                let rhs = self
                    .compile_expression(rhs)?
                    .expect("Cannot add number to unit");
                self.builder.build_int_add(lhs, rhs, "addtmp")
            }
            BinOp::Sub(lhs, rhs) => {
                let lhs = self
                    .compile_expression(lhs)?
                    .expect("Cannot add unit to number");
                let rhs = self
                    .compile_expression(rhs)?
                    .expect("Cannot add number to unit");
                self.builder.build_int_sub(lhs, rhs, "subtmp")
            }
            BinOp::Mul(lhs, rhs) => {
                let lhs = self
                    .compile_expression(lhs)?
                    .expect("Cannot add unit to number");
                let rhs = self
                    .compile_expression(rhs)?
                    .expect("Cannot add number to unit");
                self.builder.build_int_mul(lhs, rhs, "multmp")
            }
            BinOp::Div(lhs, rhs) => {
                let lhs = self
                    .compile_expression(lhs)?
                    .expect("Cannot add unit to number");
                let rhs = self
                    .compile_expression(rhs)?
                    .expect("Cannot add number to unit");
                self.builder.build_int_signed_div(lhs, rhs, "divtmp")
            }
        }
        .map(Some)
    }

    fn compile_block(&mut self, block: &Block) -> Result<Option<IntValue<'ctx>>, BuilderError> {
        if let Some(err) = block
            .body
            .iter()
            .map(|stmt| self.compile_statement(stmt))
            .find_map(Result::err)
        {
            return Err(err);
        }
        if let Some(last) = &block.last {
            self.compile_expression(last)
        } else {
            Ok(None)
        }
    }

    fn compile_statement(&mut self, stmt: &Statement) -> Result<(), BuilderError> {
        match stmt {
            Statement::Expression(expr) => self.compile_expression(expr).map(|_| ()),
            Statement::VariableDefinition(def) => self.compile_variable_definition(def),
        }
    }

    fn compile_variable_definition(
        &mut self,
        def: &VariableDefinition,
    ) -> Result<(), BuilderError> {
        let var = self
            .builder
            .build_alloca(self.context.i32_type(), &def.spec.name)?;
        if let Some(value) = &def.value {
            let value = self.compile_expression(value)?;
            if let Some(value) = value {
                self.builder.build_store(var, value)?;
            }
        }
        self.named_values.insert(
            def.spec.name.clone(),
            NamedValue::Variable(def.spec.clone(), var),
        );
        Ok(())
    }

    fn compile_loop(
        &mut self,
        initializer: Option<&Statement>,
        condition: Option<&Expression>,
        update: Option<&Statement>,
        body: &Expression,
    ) -> Result<Option<IntValue<'ctx>>, BuilderError> {
        let header = self.context.append_basic_block(
            self.current_function
                .unwrap_or_else(|| unreachable!("Can't codegen loop without a function.")),
            "loop_header",
        );
        let exit = self.context.append_basic_block(
            self.current_function
                .unwrap_or_else(|| unreachable!("Can't codegen loop without a function.")),
            "loop_exit",
        );

        self.loop_headers.push(header);
        self.loop_exits.push(exit);

        self.builder.build_unconditional_branch(header)?;
        self.builder.position_at_end(header);
        self.compile_expression(body)?;

        if header.get_terminator().is_none() {
            self.builder.build_unconditional_branch(header)?;
        }

        self.loop_headers.pop();
        self.loop_exits.pop();
        self.builder.position_at_end(exit);
        Ok(None)
    }

    fn compile_loop_flow(
        &mut self,
        typ: &Expression,
    ) -> Result<Option<IntValue<'ctx>>, BuilderError> {
        let target = match typ {
            Expression::Break => self.loop_exits.last().expect("`break` used outside of a loop"),
            Expression::Continue => self.loop_headers.last().expect("`continue` used outside of a loop"),
            _ => panic!("Loop flow must be either `break` or `continue`")
        };
        self.builder.build_unconditional_branch(*target)?;
        Ok(None)
    }
}
