extern crate rlisp;

use rlisp::parser::parse;
use rlisp::node::*;

#[test]
fn test_read_nil() {
    assert_eq!(parse("()"), Ok(rnil()));
}

#[test]
fn test_read_int() {
    assert_eq!(parse("1"), Ok(rint(1)));
    assert_eq!(parse("100"), Ok(rint(100)));
    assert_eq!(parse("1000"), Ok(rint(1000)));
}

#[test]
fn test_read_bool() {
    assert_eq!(parse("#t"), Ok(rtrue()));
    assert_eq!(parse("#f"), Ok(rfalse()));
}

#[test]
fn test_read_quote() {
    assert_eq!(parse("'()"), Ok(rquote(rnil())));
    assert_eq!(parse("'1"), Ok(rquote(rint(1))));
    assert_eq!(parse("'(1)"), Ok(rquote(rcell(rint(1), rnil()))));
    assert_eq!(parse("'(a b c)"), Ok(rquote(rcell(rsym("a"), rlist(rsym("b"), rsym("c"))))));
}

#[test]
fn test_read_cond_expr() {
    assert_eq!(parse("(= 1 2)"), Ok(rcell(rsym("="), rlist(rint(1), rint(2)))));
    assert_eq!(parse("(< 1 2)"), Ok(rcell(rsym("<"), rlist(rint(1), rint(2)))));
    assert_eq!(parse("(> 1 2)"), Ok(rcell(rsym(">"), rlist(rint(1), rint(2)))));
    assert_eq!(parse("(>= 1 2)"), Ok(rcell(rsym(">="), rlist(rint(1), rint(2)))));
    assert_eq!(parse("(<= 1 2)"), Ok(rcell(rsym("<="), rlist(rint(1), rint(2)))));
}

#[test]
fn test_read_expr() {
    // add
    assert_eq!(parse("(+ 1 2)"), Ok(rcell(rsym("+"), rlist(rint(1), rint(2)))));
    assert_eq!(parse("(+ 1 2 3)"), Ok(rcell(rsym("+"), rcell(rint(1), rlist(rint(2), rint(3))))));
    assert_eq!(parse("(+ 1 (+ 2 3))"),
               Ok(rcell(rsym("+"), rlist(rint(1), rcell(rsym("+"), rlist(rint(2), rint(3)))))));

    // sub
    assert_eq!(parse("(- 2 1)"), Ok(rcell(rsym("-"), rlist(rint(2), rint(1)))));
    assert_eq!(parse("(- (- 10 5) (- 10 5))"),
               Ok(rcell(rsym("-"), rlist(rcell(rsym("-"), rlist(rint(10), rint(5))),
                                         rcell(rsym("-"), rlist(rint(10), rint(5)))))));
}

#[test]
fn test_read_symbol() {
    assert_eq!(parse("(inc 1)"), Ok(rlist(rsym("inc"), rint(1))));
}

#[test]
fn test_read_define() {
    assert_eq!(parse("(define x 1)"), Ok(rcell(rsym("define"), rlist(rsym("x"), rint(1)))));
    assert_eq!(parse("(define x (+ 1 2))"),
               Ok(rcell(rsym("define"), rlist(rsym("x"), rcell(rsym("+"), rlist(rint(1), rint(2)))))));
}

#[test]
fn test_read_progn() {
    assert_eq!(parse("(progn (+ 1 2) (+ 1 2))"),
               Ok(rcell(rsym("progn"), rlist(rcell(rsym("+"), rlist(rint(1), rint(2))),
                                             rcell(rsym("+"), rlist(rint(1), rint(2)))))));
}

#[test]
fn test_read_if() {
    assert_eq!(parse("(if #t 1 2)"),
               Ok(rcell(rsym("if"), rcell(rtrue(), rlist(rint(1), rint(2))))));

    assert_eq!(parse("(if #t (+ 1 2) 2)"),
               Ok(rcell(rsym("if"), rcell(rtrue(), rlist(rcell(rsym("+"), rlist(rint(1), rint(2))), rint(2))))));
}

#[test]
fn test_read_lambda() {
    assert_eq!(parse("(lambda () 1)"),
               Ok(rcell(rsym("lambda"), rlist(rnil(), rint(1)))));
    assert_eq!(parse("(lambda (x) x)"),
               Ok(rcell(rsym("lambda"), rlist(rcell(rsym("x"), rnil()), rsym("x")))));
    assert_eq!(parse("(lambda () 1 2)"),
               Ok(rcell(rsym("lambda"), rcell(rnil(), rlist(rint(1), rint(2))))));
    assert_eq!(parse("(lambda (a b) (+ a b))"),
               Ok(rcell(rsym("lambda"), rlist(rlist(rsym("a"), rsym("b")),
                                              rcell(rsym("+"), rlist(rsym("a"), rsym("b")))))));
}

#[test]
fn test_read_let() {
    assert_eq!(parse("(let ((a 10)) a)"),
               Ok(rcell(rsym("let"), rlist(rcell(rlist(rsym("a"), rint(10)), rnil()), rsym("a"))))
    );

    assert_eq!(parse("(let ((a 10) (b 11)) (- a b))"),
               Ok(rcell(rsym("let"),
                        rlist(rlist(rlist(rsym("a"), rint(10)), rlist(rsym("b"), rint(11))),
                              rcell(rsym("-"), rlist(rsym("a"), rsym("b")))))));
}
