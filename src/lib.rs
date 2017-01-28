pub mod parser;
pub mod evaluator;
pub mod printer;
pub mod node;
pub mod env;
pub mod primitives;
pub mod error;

use std::rc::Rc;
use node::{Node, Prim, prim};
use env::Env;
use error::{RResult, RLispError};

fn register_symbols(env: &mut Env<Node>) {
    env.register("+", prim(Prim::Proc(Rc::new(primitives::prim_add))));
    env.register("-", prim(Prim::Proc(Rc::new(primitives::prim_sub))));
    env.register("*", prim(Prim::Proc(Rc::new(primitives::prim_mul))));
    env.register("=", prim(Prim::Proc(Rc::new(primitives::prim_eq))));
    env.register("define", prim(Prim::Proc(Rc::new(primitives::prim_define))));
    env.register("progn", prim(Prim::Proc(Rc::new(primitives::prim_progn))));
    env.register("quote", prim(Prim::Proc(Rc::new(primitives::prim_quote))));
    env.register("if", prim(Prim::Proc(Rc::new(primitives::prim_if))));
    env.register("lambda", prim(Prim::Proc(Rc::new(primitives::prim_lambda))));
    env.register("let", prim(Prim::Proc(Rc::new(primitives::prim_let))));
}

fn init(env: &mut Env<Node>) {
    register_symbols(env);
}

pub fn run<T: Into<String>>(input: T) -> RResult<Node, RLispError> {
    let renv = &mut env::Env::new();
    init(renv);

    let ast = try!(parser::parse(input).map_err( |v| RLispError::ParseError(v)));
    evaluator::eval(renv, &ast).map_err( |v| RLispError::EvalError(v) )
}
