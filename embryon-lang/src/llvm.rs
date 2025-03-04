use llvm_sys::prelude::*;

pub struct Context {
    context: LLVMContextRef,
}

impl Context {
    pub fn new() -> Self {
        Self {
            context: unsafe { llvm_sys::core::LLVMContextCreate() },
        }
    }

    pub fn module(&self, name: &str) -> Module {
        Module::new(self, name)
    }

    pub fn builder(&self) -> Builder {
        Builder::new(self)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            context: unsafe { llvm_sys::core::LLVMGetGlobalContext() },
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            llvm_sys::core::LLVMContextDispose(self.context);
        }
    }
}

pub struct Module {
    module: LLVMModuleRef,
}

impl Module {
    fn new(context: &Context, name: &str) -> Self {
        let name = std::ffi::CString::new(name).expect("Couldn't convert name to CString");
        Self {
            module: unsafe {
                llvm_sys::core::LLVMModuleCreateWithNameInContext(name.as_ptr(), context.context)
            },
        }
    }

    pub fn set_target(&self, target: &str) {
        let target = std::ffi::CString::new(target).expect("Couldn't convert target to CString");
        unsafe {
            llvm_sys::core::LLVMSetTarget(self.module, target.as_ptr());
        }
    }

    pub fn write_bitcode(&self, filename: &str) {
        let filename =
            std::ffi::CString::new(filename).expect("Couldn't convert filename to CString");
        unsafe {
            llvm_sys::bit_writer::LLVMWriteBitcodeToFile(self.module, filename.as_ptr());
        }
    }

    pub fn add_function(&self, name: &str, function_type: FunctionType) -> Function {
        let name = std::ffi::CString::new(name).expect("Couldn't convert name to CString");
        let func =
            unsafe { llvm_sys::core::LLVMAddFunction(self.module, name.as_ptr(), function_type.0) };
        Function(func)
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            llvm_sys::core::LLVMDisposeModule(self.module);
        }
    }
}

pub struct Builder {
    builder: LLVMBuilderRef,
}

impl Builder {
    fn new(context: &Context) -> Self {
        Self {
            builder: unsafe { llvm_sys::core::LLVMCreateBuilderInContext(context.context) },
        }
    }

    pub fn position_at_end(&self, block: Block) {
        unsafe {
            llvm_sys::core::LLVMPositionBuilderAtEnd(self.builder, block.0);
        }
    }

    pub fn return_(&self, value: Value) {
        unsafe {
            llvm_sys::core::LLVMBuildRet(self.builder, value.into());
        }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            llvm_sys::core::LLVMDisposeBuilder(self.builder);
        }
    }
}

pub enum Value {
    ConstU32(u32),
    Function(Function),
}

impl From<Value> for LLVMValueRef {
    fn from(value: Value) -> LLVMValueRef {
        match value {
            Value::ConstU32(value) => unsafe {
                llvm_sys::core::LLVMConstInt(Type::Int32.into(), value as u64, 0)
            },
            Value::Function(function) => function.0,
        }
    }
}

pub struct Function(LLVMValueRef);

impl Function {
    pub fn append_basic_block(&self, name: &str) -> Block {
        let name = std::ffi::CString::new(name).expect("Couldn't convert name to CString");
        Block(unsafe {
            llvm_sys::core::LLVMAppendBasicBlockInContext(
                llvm_sys::core::LLVMGetGlobalContext(),
                self.0,
                name.as_ptr(),
            )
        })
    }
}

pub enum Type {
    Int32,
    Function(FunctionType),
}

impl From<Type> for LLVMTypeRef {
    fn from(t: Type) -> LLVMTypeRef {
        match t {
            Type::Int32 => unsafe { llvm_sys::core::LLVMInt32Type() },
            Type::Function(function) => function.0,
        }
    }
}

pub struct FunctionType(LLVMTypeRef);

impl FunctionType {
    pub fn new(returns: Type, params: Vec<Type>) -> Self {
        let params_len = params.len();
        let params_ptr: Vec<LLVMTypeRef> = params.into_iter().map(|t| t.into()).collect();
        Self(unsafe {
            llvm_sys::core::LLVMFunctionType(
                returns.into(),
                params_ptr.as_ptr() as *mut _,
                params_len as u32,
                0,
            )
        })
    }
}

pub struct Block(LLVMBasicBlockRef);
