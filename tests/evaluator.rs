extern crate rlisp;

use std::rc::Rc;
use rlisp::primitives::*;
use rlisp::evaluator::eval;
use rlisp::env::Env;
use rlisp::node::*;

fn test_init(env: &mut Env<Node>) {
    env.register("+", Node::Prim(Prim(Rc::new(prim_add))));
    env.register("-", Node::Prim(Prim(Rc::new(prim_sub))));
}

#[test]
fn test_eval_int() {
    let env = &mut Env::new();
    // 1
    assert_eq!(eval(env, &rint(1)).unwrap(), rint(1));
}

#[test]
fn test_eval_add_prim() {
    let env = &mut Env::new();
    test_init(env);
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
    test_init(env);
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
    test_init(env);
    env.register("define", Node::Prim(Prim(Rc::new(prim_define))));
    // (define x 1)
    let t1 = rcell(rsym("define"), rcell(rsym("x"), rcell(rint(1), rnil())));
    // (define y (+ 1 2))
    let t2 = rcell(rsym("define"), rcell(rsym("y"), rcell(rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil()))), rnil())));

    assert_eq!(eval(env, &t1).unwrap(), rint(1));
    assert_eq!(eval(env, &t2).unwrap(), rint(3));

    assert_eq!(*(env.find("x").unwrap()), rint(1));
    assert_eq!(*(env.find("y").unwrap()), rint(3));
}


#[test]
fn test_eval_progn_prim() {
    let env = &mut Env::new();
    test_init(env);
    env.register("progn", Node::Prim(Prim(Rc::new(prim_progn))));
    env.register("define", Node::Prim(Prim(Rc::new(prim_define))));

    // (progn 1 2)
    let t1 = rcell(rsym("progn"), rcell(rint(1), rcell(rint(2), rnil())));
    // (progn (+ 1 2) (+ 3 2))
    let t2 = rcell(rsym("progn"),
                   rcell(rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil()))),
                         rcell(rcell(rsym("+"), rcell(rint(3), rcell(rint(2), rnil()))), rnil())));

    // (progn (define x 10) (+ x 10))
    let t3 = rcell(rsym("progn"),
                   rcell(rcell(rsym("define"), rcell(rsym("x"), rcell(rint(10), rnil()))),
                         rcell(rcell(rsym("+"), rcell(rsym("x"), rcell(rint(10), rnil()))), rnil())));

    assert_eq!(eval(env, &t1).unwrap(), rint(2));
    assert_eq!(eval(env, &t2).unwrap(), rint(5));
    assert_eq!(eval(env, &t3).unwrap(), rint(20));
}
