extern crate llvm_sys as llvm;
use std::collections::hash_map::Entry;
use std::ffi::CString;
use std::ptr;
use self::llvm::prelude::*;

use node::*;
use env::Env;

#[derive(Clone)]
pub enum Value {
    Int(LLVMValueRef),
    Function(LLVMValueRef), // to fix
    Lambda(Env<Value>, Node, Node), // env, args, body
}

impl Value {
    pub fn to_ref(&self) -> LLVMValueRef {
        match self {
            &Value::Int(v) => v,
            &Value::Function(v) => v,
            _ => panic!("TBC"),
        }
    }

    pub fn create_from(&self, v: LLVMValueRef) -> Value {
        match self {
            &Value::Int(_) => Value::Int(v),
            &Value::Function(_) => Value::Function(v),
            _ => panic!("TBC"),
        }
    }
}

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
        // alias
        e.register("+", "prim_add".to_string());
        e.register("-", "prim_sub".to_string());
        e.register("*", "prim_mul".to_string());
        e.register("/", "prim_div".to_string());

        VM {
            context: context,
            builder: VM::create_builder_in_context(context),
            module: VM::create_module_with_name("rlisp"),
            int_value_type: VM::int_type(context),
            prims: e.clone(),
        }
    }

    pub fn run(&self, node: &Node) {
        let env = &mut Env::new();
        self.init(env);
        self.pre_gen(node, env);
        self.llvm_ret(self.codegen(node, env).to_ref());
        self.finalize();
    }

    fn init(&self, env: &mut Env<Value>) {
        self.register_symbols(env);

        // create main
        self.create_fun_and_set_bb("main", self.int_value_type, &mut []);
    }

    fn pre_gen(&self, ast: &Node, env: &mut Env<Value>) {
        if let &Node::Cell(ref car, ref cdr) = ast {
            if rnil() == **cdr {
                self.pre_gen(car, env);
                return;
            }

            match sym_to_str(car) {
                Ok(x) => {
                    match x.as_ref() {
                        "define" => self.prim_define(&(**cdr), env),
                        _ => self.pre_gen(cdr, env),
                    }
                }
                Err(_) => {
                    self.pre_gen(car, env);
                    self.pre_gen(cdr, env);
                }
            }
        }
    }

    fn prim_define(&self, body: &Node, env: &mut Env<Value>) {
        if let Node::Cell(ref ncar, ref ncdr) = *body {
            let sym_name = sym_to_str(ncar).unwrap();
            let val = self.codegen(&rcar(ncdr).unwrap().clone(), env);
            let ptr = match env.entry(sym_name.as_ref()) {
                Entry::Occupied(o) => o.get().clone(),
                Entry::Vacant(v) => {
                    match val {
                        Value::Lambda(_, _, _) => {
                            v.insert(val);
                            return; // do not call store operator of llvm for lambda
                        }
                        _ => {
                            let p = self.allocate_mem(sym_name.as_ref(), self.int_value_type);
                            v.insert(val.create_from(p)).clone()
                        }
                    }
                }
            };
            self.llmv_store(val.to_ref(), ptr.to_ref());
        }
    }

    fn register_symbols(&self, env: &mut Env<Value>) {
        env.register("+", self.prim_arith("prim_add"));
        env.register("-", self.prim_arith("prim_sub"));
        env.register("*", self.prim_arith("prim_mul"));
        env.register("/", self.prim_arith("prim_div"));
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

    fn prim_arith(&self, name: &str) -> Value {
        let ty = self.int_value_type;
        let arg_types = &mut [ty, ty];
        let fun = self.create_fun_and_set_bb(name, ty, arg_types);
        let lh = self.get_param_fun(&fun, 0);
        let rh = self.get_param_fun(&fun, 1);
        let v = match name {
            "prim_add" => self.llvm_add(lh, rh),
            "prim_sub" => self.llvm_sub(lh, rh),
            "prim_mul" => self.llvm_mul(lh, rh),
            "prim_div" => self.llvm_div(lh, rh),
            _ => {
                println!("{:?}", name);
                panic!("not support arith")
            }
        };
        self.llvm_ret(v);
        Value::Function(fun)
    }

    fn llvm_ret(&self, ret: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildRet(self.builder, ret) }
    }

    fn llvm_add(&self, lh: LLVMValueRef, rh: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildAdd(self.builder, lh, rh, cptr!("v")) }
    }

    fn llvm_sub(&self, lh: LLVMValueRef, rh: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildSub(self.builder, lh, rh, cptr!("v")) }
    }

    fn llvm_mul(&self, lh: LLVMValueRef, rh: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildMul(self.builder, lh, rh, cptr!("v")) }
    }

    fn llvm_div(&self, lh: LLVMValueRef, rh: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildUDiv(self.builder, lh, rh, cptr!("v")) }
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

    fn codegen(&self, ast: &Node, env: &mut Env<Value>) -> Value {
        match *ast {
            Node::Int(val) => Value::Int(self.int_value(val as u64)),
            Node::Cell(ref car, ref cdr) => {
                match **car {
                    Node::Sym(ref n) => {
                        //
                        self.apply_fun(env, n, cdr)
                    }
                    Node::Cell(_, _) => {
                        let lam = self.codegen(car, env);
                        match lam {
                            Value::Lambda(ref new_env, ref args, ref body) => {
                                self.codegen_lambda(&mut new_env.clone(),
                                                    &args,
                                                    &body,
                                                    cdr,
                                                    &mut env.clone())
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => {
                        println!("{:?}", car);
                        panic!("not suport")
                    }
                }
            }
            Node::Sym(ref name) => {
                let val = env.find(name).unwrap();
                val.create_from(self.build_load(val.to_ref(), name))
            }
            ref a => {
                println!("{:?}", a);
                panic!("not support in codegen")
            }
        }
    }

    fn apply_fun(&self, env: &mut Env<Value>, name: &str, rest: &Node) -> Value {
        match name {
            "+" | "-" | "*" | "/" => self.codegen_arith(name, rest, env),
            "define" => {
                let c = car_ref(rest).and_then(|v| sym_to_str(v)).unwrap();
                match env.clone().entry(c.as_ref()) {
                    Entry::Occupied(o) => {
                        match o.get() {
                            &Value::Lambda(_, _, _) => Value::Int(self.int_value(10)), // tmp
                            _ => self.codegen(&Node::Sym(c.into()), env),
                        }
                    }
                    Entry::Vacant(v) => panic!("not supoprt"),
                }
            }
            "progn" => {
                env.push_local_scope();
                let mut vec = node_to_vec(rest.clone());
                let vec2 = vec.split_off(1);
                let mut ret = self.codegen(&vec[0], env);
                for v in vec2 {
                    ret = self.codegen(&v.clone(), env);
                }
                env.pop_local_scope();
                ret
            }
            "lambda" => {
                env.push_local_scope();
                let ca = rcar(rest).unwrap();
                let cd = rcdr(rest).unwrap();
                let lam = Value::Lambda(env.clone(), ca, cd);
                env.push_local_scope();
                lam
            }
            "let" => {
                env.push_local_scope();
                let ret = self.codegen_let(env, rest);
                env.push_local_scope();
                ret
            }
            _ => {
                match env.find(name) {
                    Some(v) => {
                        match *v {
                            Value::Lambda(ref new_env, ref args, ref body) => {
                                self.codegen_lambda(&mut new_env.clone(),
                                                    &args,
                                                    &body,
                                                    rest,
                                                    &mut env.clone())
                            }
                            _ => unreachable!(),
                        }
                    }
                    None => {
                        println!("at apply fun {:?}", name);
                        panic!("unknow");
                    }
                }
            }
        }
    }

    fn codegen_let(&self, env: &mut Env<Value>, lst: &Node) -> Value {
        let args = car_ref(lst).unwrap();
        let body = cdr_ref(lst).and_then(|v| car_ref(v)).unwrap();
        let args = node_to_vec(args.clone());

        for n in args.iter() {
            let key: &str = car_ref(n).and_then(|n| sym_to_str(n)).unwrap();
            let val: &Node = cdr_ref(n).and_then(|v| car_ref(v)).unwrap();
            let v = self.codegen(val, env);
            match v {
                Value::Lambda(_, _, _) => {
                    env.register(key, v);
                }
                _ => {
                    let p = self.allocate_mem(key.as_ref(), self.int_value_type);
                    env.register(key, v.create_from(p));
                    self.llmv_store(v.to_ref(), p);
                }
            }
        }

        self.apply_fun(env, "progn", &rcell(body.clone(), rnil()))
    }

    fn codegen_lambda(&self,
                      lambda_env: &mut Env<Value>,
                      vargs: &Node,
                      body: &Node,
                      aargs: &Node,
                      env: &mut Env<Value>)
                      -> Value {
        let arg_values = self.codegen_list(env, aargs);
        let vargs = node_to_vec(vargs.clone());

        for (a, name) in arg_values.iter().zip(vargs.iter()) {
            let n = sym_to_str(name).unwrap();
            match a {
                &Value::Lambda(_, _, _) => {
                    lambda_env.register(n, a.clone());
                }
                _ => {
                    let p = self.allocate_mem(n.as_ref(), self.int_value_type);
                    lambda_env.register(n, a.create_from(p));
                    self.llmv_store(a.to_ref(), p);
                }
            }
        }

        self.apply_fun(lambda_env, "progn", body)
    }

    fn codegen_list(&self, env: &mut Env<Value>, n: &Node) -> Vec<Value> {
        let args = node_to_vec(n.clone());
        args.iter().map(|a| self.codegen(a, env)).collect() // side effect...
    }

    fn codegen_arith(&self, fname: &str, rest: &Node, env: &mut Env<Value>) -> Value {
        match rest {
            &Node::Cell(ref car, ref cdr) => {
                let lh = self.codegen(car, env); // to fix
                if **cdr == rnil() {
                    return lh;
                }
                let rh = self.codegen_arith(fname, cdr, env);
                let v = &fname.into();
                let name = self.prims.find(fname).unwrap_or(v);
                let fun = self.find_function(name).unwrap();
                let args = &mut Vec::new();
                match (lh, rh) {
                    (Value::Int(v), Value::Int(v2)) => {
                        args.push(v);
                        args.push(v2);
                    }
                    _ => panic!("Wrong type of argument"),
                }
                Value::Int(self.build_call(fun, args, 2, "v"))
            }
            _ => panic!("siran"),
        }
    }

    fn find_function(&self, name: &str) -> Option<LLVMValueRef> {
        let v = unsafe { llvm::core::LLVMGetNamedFunction(self.module, cptr!(name)) };
        let is_null = unsafe { llvm::core::LLVMIsNull(v) > 0 };
        if is_null { None } else { Some(v) }
    }

    fn build_call(&self,
                  fun: LLVMValueRef,
                  args: &mut [LLVMValueRef],
                  args_count: u32,
                  name: &str)
                  -> LLVMValueRef {
        unsafe {
            llvm::core::LLVMBuildCall(self.builder,
                                      fun,
                                      args.as_mut_ptr(),
                                      args_count,
                                      cptr!(name))
        }
    }

    fn int_value(&self, val: u64) -> LLVMValueRef {
        unsafe { llvm::core::LLVMConstInt(self.int_value_type, val, 0) }
    }

    fn allocate_mem(&self, name: &str, typ: LLVMTypeRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildAlloca(self.builder, typ, cptr!(name)) }
    }


    fn build_load(&self, ptr: LLVMValueRef, name: &str) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildLoad(self.builder, ptr, cptr!(name)) }
    }

    fn llmv_store(&self, val: LLVMValueRef, target: LLVMValueRef) -> LLVMValueRef {
        unsafe { llvm::core::LLVMBuildStore(self.builder, val, target) }
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
