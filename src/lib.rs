pub mod parser;
pub mod evaluator;
pub mod printer;
pub mod node;
pub mod env;
pub mod primitives;

use std::rc::Rc;
use node::{Node, Prim, prim};
use env::Env;

fn register_symbols(env: &mut Env<Node>) {
    env.register("+", prim(Prim::Proc(Rc::new(primitives::prim_add))));
    env.register("-", prim(Prim::Proc(Rc::new(primitives::prim_sub))));
    env.register("define", prim(Prim::Proc(Rc::new(primitives::prim_define))));
    env.register("progn", prim(Prim::Proc(Rc::new(primitives::prim_progn))));
    env.register("quote", prim(Prim::Proc(Rc::new(primitives::prim_quote))));
    env.register("if", prim(Prim::Proc(Rc::new(primitives::prim_if))));
    env.register("lambda", prim(Prim::Proc(Rc::new(primitives::prim_lambda))));
    env.register("let", prim(Prim::Proc(Rc::new(primitives::prim_let))));
}

fn init(env: &mut Env<Node>) {
    register_symbols(env);

    // this method should be call at the last of initilization.
    env.push_local_scope();
}

pub fn run(input: &str) ->  Result<Node, String> {    // specific type
    let renv = &mut env::Env::new();
    init(renv);
    match parser::parse(input) {
        Ok(result) => evaluator::eval(renv, &result).map_err( |_| "Evaluate Error".into()),
        Err(v) => Err(v.to_str()),
    }
}
