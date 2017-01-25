extern crate rlisp;

use std::rc::Rc;
use rlisp::primitives::*;
use rlisp::evaluator::eval;
use rlisp::env::Env;
use rlisp::node::*;

#[test]
fn test_eval_int() {
    let env = &mut Env::new();
    // 1
    assert_eq!(eval(env, &rint(1)).unwrap(), rint(1));
}

#[test]
fn test_eval_add_prim() {
    let env = &mut Env::new();
    env.register("+", Node::Prim(Prim(Rc::new(prim_add))));
    // (+ 1 2)
    assert_eq!(eval(env, &rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil())))).unwrap(), rint(3));
    // (+ 1 2 3)
    assert_eq!(eval(env, &rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rcell(rint(3), rnil()))))).unwrap(), rint(6));
    // (+ 1 (+ 2 3))
    assert_eq!(eval(env, &rcell(rsym("+"),
                                rcell(rint(1),
                                      rcell(rcell(rsym("+"), rcell(rint(2), rcell(rint(3), rnil()))), rnil())))).unwrap(),
               rint(6));
}
