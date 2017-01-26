extern crate rlisp;

use std::rc::Rc;
use rlisp::primitives::*;
use rlisp::evaluator::eval;
use rlisp::env::Env;
use rlisp::node::*;

fn test_init(env: &mut Env<Node>) {
    env.register("+", Node::Prim(Prim::Proc(Rc::new(prim_add))));
    env.register("-", Node::Prim(Prim::Proc(Rc::new(prim_sub))));
    env.register("if", Node::Prim(Prim::Proc(Rc::new(prim_if))));
    env.register("quote", Node::Prim(Prim::Proc(Rc::new(prim_quote))));
    env.register("lambda", Node::Prim(Prim::Proc(Rc::new(prim_lambda))));
    env.register("progn", Node::Prim(Prim::Proc(Rc::new(prim_progn))));
    env.register("define", Node::Prim(Prim::Proc(Rc::new(prim_define))));
    env.register("let", Node::Prim(Prim::Proc(Rc::new(prim_let))));
}

#[test]
fn test_eval_int() {
    let env = &mut Env::new();
    // 1
    assert_eq!(eval(env, &rint(1)).unwrap(), rint(1));
}

#[test]
fn test_eval_bool() {
    let env = &mut Env::new();
    test_init(env);
    // #t
    assert_eq!(eval(env, &rtrue()).unwrap(), rtrue());
    // #f
    assert_eq!(eval(env, &rfalse()).unwrap(), rfalse());
}

#[test]
fn test_eval_quote() {
    let env = &mut Env::new();
    test_init(env);
    // '()
    assert_eq!(eval(env, &rquote(rnil())).unwrap(), rnil());
    // '1
    assert_eq!(eval(env, &rquote(rint(1))).unwrap(), rint(1));
    // '#t
    assert_eq!(eval(env, &rquote(rtrue())).unwrap(), rtrue());
    // '(1)
    assert_eq!(eval(env, &rquote(rcell(rint(1), rnil()))).unwrap(), rcell(rint(1), rnil()));
    // '(a b c)
    assert_eq!(eval(env, &rquote(rcell(rint(1), rnil()))).unwrap(), rcell(rint(1), rnil()));
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

#[test]
fn test_eval_if_prim() {
    let env = &mut Env::new();
    test_init(env);
    // (if #t (+ 1 2) 2)
    let t1 = rcell(rsym("if"), rcell(rtrue(), rcell(rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil()))), rcell(rint(2), rnil()))));
    // (if #f 2 (+ 1 2))
    let t2 = rcell(rsym("if"), rcell(rfalse(), rlist(rint(2), rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil()))))));
    // (if 1 2 (+ 1 2))
    let t3 = rcell(rsym("if"), rcell(rint(1), rlist(rint(2), rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil()))))));
    // (if () 2 (+ 1 2))
    let t4 = rcell(rsym("if"), rcell(rnil(), rlist(rint(2), rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil()))))));

    assert_eq!(eval(env, &t1).unwrap(), rint(3));
    assert_eq!(eval(env, &t2).unwrap(), rint(3));
    assert_eq!(eval(env, &t3).unwrap(), rint(2));
    assert_eq!(eval(env, &t4).unwrap(), rint(2)); // nil is #t
}

#[test]
fn test_eval_lambda_prim() {
    let env = &mut Env::new();
    test_init(env);
    // (lambda () 1)
    let t1 = rcell(rcell(rsym("lambda"), rcell(rnil(), rcell(rint(1), rnil()))), rnil());
    // (lambda () 1 2)
    let t2 = rcell(rcell(rsym("lambda"), rcell(rnil(), rcell(rint(1), rcell(rint(2), rnil())))), rnil());
    // ((lambda (x) (+ x 1)) 10)
    let t3 = rcell(
        rcell(rsym("lambda"), rcell(rcell(rsym("x"), rnil()), rcell(rcell(rsym("+"), rcell(rsym("x"), rcell(rint(1), rnil()))), rnil()))),
        rcell(rint(10), rnil()));

    // ((lambda (a b) (- a b)) 10 5)
    let t4 = rcell(
        rcell(rsym("lambda"), rcell(rcell(rsym("a"), rcell(rsym("b"), rnil())), rcell(rcell(rsym("-"), rcell(rsym("a"), rcell(rsym("b"), rnil()))), rnil()))),
        rcell(rint(10), rcell(rint(5), rnil())));

    assert_eq!(eval(env, &t1).unwrap(), rint(1));
    assert_eq!(eval(env, &t2).unwrap(), rint(2));
    assert_eq!(eval(env, &t3).unwrap(), rint(11));
    assert_eq!(eval(env, &t4).unwrap(), rint(5));
}

#[test]
fn test_eval_let_prim() {
    let env = &mut Env::new();
    test_init(env);
    // (let ((a 10)) a)
    let t1 = rcell(rsym("let"), rlist(rcell(rlist(rsym("a"), rint(10)), rnil()), rsym("a")));
    // (let ((a 10) (b 11)) (- a b))
    let t2 = rcell(rsym("let"),
                     rlist(rlist(rlist(rsym("a"), rint(10)), rlist(rsym("b"), rint(11))),
                           rcell(rsym("-"), rlist(rsym("a"), rsym("b")))));

    assert_eq!(eval(env, &t1).unwrap(), rint(10));
    assert_eq!(eval(env, &t2).unwrap(), rint(-1));

}
