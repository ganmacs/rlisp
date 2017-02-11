extern crate llvm_sys as llvm;
use std::ffi::CString;
use std::ptr;
use self::llvm::prelude::*;

use node::*;
use env::Env;

pub struct VM {
    context: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
    int_value_type: LLVMTypeRef,
    prims: Env<String>,
}

macro_rules! cptr {
    ($x: expr) => (CString::new($x).unwrap().as_ptr())
}

impl VM {
    pub fn new() -> VM {
        let context = VM::create_context();
        let e = &mut Env::new();
        e.register("+", "prim_plus".to_string());
        VM {
            context: context,
            builder: VM::create_builder_in_context(context),
            module: VM::create_module_with_name("rlisp"),
            int_value_type: VM::int_type(context),
            prims: e.clone(),
        }
    }

    pub fn run(&self, node: &Node) {
        self.init();
        let env = &mut Env::new();
        self.llvm_ret(self.codegen(node, env));
        self.finalize();
    }

    fn init(&self) {
        self.add_fun();

        // create main
        self.create_fun_and_set_bb("main", self.int_value_type, &mut []);
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

    // int -> int -> int
    fn add_fun(&self) -> LLVMValueRef {
        let ty = self.int_value_type;
        let arg_types = &mut [ty, ty];
        let name = self.prims.find("+").unwrap();
        let fun = self.create_fun_and_set_bb(name, ty, arg_types);
        let lh = self.get_param_fun(&fun, 0);
        let rh = self.get_param_fun(&fun, 1);
        let v = self.llvm_add(lh, rh);
        self.llvm_ret(v);
        fun
    }

    fn llvm_ret(&self, ret: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildRet(self.builder, ret) }
    }

    fn llvm_add(&self, lh: LLVMValueRef, rh: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildAdd(self.builder, lh, rh, cptr!("v")) }
    }

    fn get_param_fun(&self, func: &LLVMValueRef, i: u32) -> LLVMValueRef {
        unsafe { llvm::core::LLVMGetParam(*func, i) }
    }

    fn count_params_fun(&self, func: &LLVMValueRef) -> u32 {
        unsafe { llvm::core::LLVMCountParams(*func) }
    }

    fn get_params_fun(&self, fun: &LLVMValueRef) -> Vec<LLVMValueRef> {
        let count = self.count_params_fun(fun) as usize;
        let p = {
            let buf = &mut Vec::with_capacity(count);
            buf.as_mut_ptr()
        };
        unsafe {
            // std::mem::forget(buf);
            llvm::core::LLVMGetParams(*fun, p);
            Vec::from_raw_parts(p, count, count)
        }
    }

    fn create_fun_and_set_bb(&self,
                             name: &str,
                             ret_ty: LLVMTypeRef,
                             arg_types: &mut [LLVMTypeRef])
                             -> LLVMValueRef {
        let fun_type = self.get_function_type(ret_ty, arg_types);
        let fun = self.add_function(name, fun_type);
        let bb = self.append_basic_block("entry", fun);
        self.set_builder_position_at_end(bb);
        fun
    }

    fn codegen(&self, ast: &Node, env: &mut Env<LLVMValueRef>) -> LLVMValueRef {
        println!("{:?}", ast);
        match *ast {
            Node::Int(val) => {
                println!("{:?}", val);
                self.int_value(val as u64)
            }
            Node::Cell(ref car, ref cdr) => {
                match **car {
                    Node::Sym(ref n) => self.apply_fun(env, n, cdr),
                    _ => panic!("not suport"),
                }
            }
            Node::Sym(ref v) => *env.find(v).unwrap(),  // TODO fix
            ref a => {
                println!("{:?}", a);
                panic!("not support in codegen")
            }
        }
    }

    fn apply_fun(&self, env: &mut Env<LLVMValueRef>, name: &str, rest: &Node) -> LLVMValueRef {
        match name {
            "+" => self.call_add_fun(env, rest),
            _ => panic!("unknow"),
        }
    }

    fn call_add_fun(&self, env: &mut Env<LLVMValueRef>, rest: &Node) -> LLVMValueRef {
        match rest {
            &Node::Cell(ref car, ref cdr) => {
                let lh = self.codegen(car, env);
                if **cdr == rnil() {
                    return lh;
                }
                let rh = self.apply_fun(env, "+", cdr);
                let name = self.prims.find("+").unwrap(); // alias
                let fun = self.find_function(name).unwrap();
                let v = &mut Vec::new();
                let args = &mut {
                    v.push(lh);
                    v.push(rh);
                    v
                };
                unsafe {
                    llvm::core::LLVMBuildCall(self.builder, fun, args.as_mut_ptr(), 2, cptr!("v"))
                }

            }
            _ => panic!("siran"),
        }
    }

    fn codegen_list(&self, env: &mut Env<LLVMValueRef>, n: &Node) -> Vec<LLVMValueRef> {
        let args = &mut Vec::new();
        let mut node = n;
        while *node != rnil() {
            match node {
                &Node::Cell(ref car, ref cdr) => {
                    args.push(self.codegen(car, env));
                    node = cdr
                }
                _ => panic!("hgoe"),
            }
        }
        args.clone()
    }

    fn apply_codegen(&self,
                     env: &mut Env<LLVMValueRef>,
                     fun: LLVMValueRef,
                     rest: &Node)
                     -> LLVMValueRef {
        let args = self.codegen_list(env, rest);
        let mut_args = args.clone().as_mut_ptr();
        unsafe {
            llvm::core::LLVMBuildCall(self.builder, fun, mut_args, args.len() as u32, cptr!("v"))
        }
    }

    fn find_function(&self, name: &str) -> Option<LLVMValueRef> {
        let v = unsafe { llvm::core::LLVMGetNamedFunction(self.module, cptr!(name)) };
        let is_null = unsafe { llvm::core::LLVMIsNull(v) > 0 };
        if is_null { None } else { Some(v) }
    }


    fn int_value(&self, val: u64) -> LLVMValueRef {
        unsafe { llvm::core::LLVMConstInt(self.int_value_type, val, 0) }
    }

    fn allocate_mem(&self, name: &str, typ: LLVMTypeRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildAlloca(self.builder, typ, cptr!(name)) }
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
                         args_type: &mut [LLVMTypeRef])
                         -> LLVMTypeRef {
        let l = args_type.len() as u32;
        if l == 0 {
            unsafe { llvm::core::LLVMFunctionType(ret_type, ptr::null_mut(), 0, 0) }
        } else {
            unsafe { llvm::core::LLVMFunctionType(ret_type, args_type.as_mut_ptr(), l, 0) }
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
        unsafe { llvm::core::LLVMInt32TypeInContext(context) }
    }
}
