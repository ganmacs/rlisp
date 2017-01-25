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
    let t1 = rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil())));
    // (+ 1 2 3)
    let t2 = rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rcell(rint(3), rnil()))));
    // (+ 1 (+ 2 3))
    let t3 = rcell(rsym("+"), rcell(rint(1), rcell(rcell(rsym("+"), rcell(rint(2), rcell(rint(3), rnil()))), rnil())));

    assert_eq!(eval(env, &t1).unwrap(), rint(3));
    assert_eq!(eval(env, &t2).unwrap(), rint(6));
    assert_eq!(eval(env, &t3).unwrap(), rint(6));
}

#[test]
fn test_eval_sub_prim() {
    let env = &mut Env::new();
    env.register("-", Node::Prim(Prim(Rc::new(prim_sub))));
    // (- 2 3)
    let t1 = rcell(rsym("-"), rcell(rint(2), rcell(rint(1), rnil())));
    // (- 3 2 1)
    let t2 = rcell(rsym("-"), rcell(rint(3), rcell(rint(2), rcell(rint(1), rnil()))));
    // (- 3 (- 2 1))
    let t3 = rcell(rsym("-"), rcell(rint(3), rcell(rcell(rsym("-"), rcell(rint(2), rcell(rint(1), rnil()))), rnil())));

    assert_eq!(eval(env, &t1).unwrap(), rint(1));
    assert_eq!(eval(env, &t2).unwrap(), rint(0));
    assert_eq!(eval(env, &t3).unwrap(), rint(2));
}

#[test]
fn test_eval_define_prim() {
    let env = &mut Env::new();
    env.register("define", Node::Prim(Prim(Rc::new(prim_define))));
    env.register("+", Node::Prim(Prim(Rc::new(prim_add))));
    // (define x 1)
    let t1 = rcell(rsym("define"), rcell(rsym("x"), rcell(rint(1), rnil())));
    // (define y (+ 1 2))
    let t2 = rcell(rsym("define"), rcell(rsym("y"), rcell(rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil()))), rnil())));

    assert_eq!(eval(env, &t1).unwrap(), rint(1));
    assert_eq!(eval(env, &t2).unwrap(), rint(3));

    assert_eq!(*(env.find("x").unwrap()), rint(1));
    assert_eq!(*(env.find("y").unwrap()), rint(3));
}
