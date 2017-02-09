extern crate llvm_sys as llvm;
use std::ffi::CString;
use std::ptr;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use self::llvm::prelude::*;

use node::*;
use env::Env;

pub struct VM {
    context: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
    int_value_type: LLVMTypeRef,
    env: Env<LLVMValueRef>,
}

macro_rules! cptr {
    ($x: expr) => (CString::new($x).unwrap().as_ptr())
}

impl VM {
    pub fn new() -> VM {
        let context = VM::create_context();
        VM {
            context: context,
            builder: VM::create_builder_in_context(context),
            module: VM::create_module_with_name("rlisp"),
            int_value_type: VM::int_type(context),
            env: Env::new(),
        }
    }

    pub fn run(&self, node: &Node) {
        self.init();
        // generate code

        self.finalize();
    }

    fn init(&self) {
        let func_type = self.get_function_type(self.int_value_type, &mut Vec::new());
        let fun = self.add_function("main", func_type);
        let bb = self.append_basic_block("entry", fun);
        self.set_builder_position_at_end(bb);
    }

    fn finalize(&self) {
        unsafe {
            llvm::core::LLVMDumpModule(self.module);
            llvm::core::LLVMPrintModuleToFile(self.module, cptr!("out.ll"), ptr::null_mut());

            llvm::core::LLVMDisposeBuilder(self.builder);
            llvm::core::LLVMDisposeModule(self.module);
            llvm::core::LLVMContextDispose(self.context);
        }
    }


    fn set_builder_position_at_end(&self, bb: LLVMBasicBlockRef) {
        unsafe {
            llvm::core::LLVMPositionBuilderAtEnd(self.builder, bb);
        }
    }

    fn append_basic_block(&self, name: &str, fun: LLVMValueRef) -> LLVMBasicBlockRef {
        unsafe { llvm::core::LLVMAppendBasicBlockInContext(self.context, fun, cptr!(name)) }
    }

    fn get_function_type(&self,
                         ret_type: LLVMTypeRef,
                         args_type: &mut Vec<LLVMTypeRef>)
                         -> LLVMTypeRef {
        let l = args_type.len();
        if l == 0 {
            unsafe { llvm::core::LLVMFunctionType(ret_type, ptr::null_mut(), 0, 0) }
        } else {
            unsafe { llvm::core::LLVMFunctionType(ret_type, args_type.as_mut_ptr(), 0, 0) }
        }
    }

    fn add_function(&self, name: &str, fun_type: LLVMTypeRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMAddFunction(self.module, cptr!(name), fun_type) }
    }

    fn create_module_with_name(name: &str) -> LLVMModuleRef {
        unsafe { llvm::core::LLVMModuleCreateWithName(cptr!(name)) }
    }

    fn create_builder_in_context(context: LLVMContextRef) -> LLVMBuilderRef {
        unsafe { llvm::core::LLVMCreateBuilderInContext(context) }
    }

    fn create_context() -> LLVMContextRef {
        unsafe { llvm::core::LLVMContextCreate() }
    }

    fn int_type(context: LLVMContextRef) -> LLVMTypeRef {
        unsafe { llvm::core::LLVMInt64TypeInContext(context) }
    }
}
