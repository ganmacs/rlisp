extern crate rlisp;

use rlisp::parser::parse;
use rlisp::node::*;

#[test]
fn test_read_nil() {
    assert_eq!(parse("()").unwrap(), rnil());
}

#[test]
fn test_read_int() {
    assert_eq!(parse("1").unwrap(), rint(1));
    assert_eq!(parse("100").unwrap(), rint(100));
    assert_eq!(parse("1000").unwrap(), rint(1000));
}

#[test]
fn test_read_dot() {
    assert_eq!(parse("'(1)").unwrap(), rquote(rcell(rint(1), rnil())));
}

#[test]
fn test_read_expr() {
    // add
    assert_eq!(parse("(+ 1 2)").unwrap(),
               rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil()))));
    assert_eq!(parse("(+ 1 2 3)").unwrap(),
               rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rcell(rint(3), rnil())))));
    assert_eq!(parse("(+ 1 (+ 2 3))").unwrap(),
               rcell(rsym("+"),
                     rcell(rint(1),
                           rcell(rcell(rsym("+"), rcell(rint(2), rcell(rint(3), rnil()))), rnil()))));

    // sub
    assert_eq!(parse("(- 2 1)").unwrap(),
               rcell(rsym("-"), rcell(rint(2), rcell(rint(1), rnil()))));

    assert_eq!(parse("(- (- 10 5) (- 10 5))").unwrap(),
               rcell(rsym("-"),
                     rcell(rcell(rsym("-"), rcell(rint(10), rcell(rint(5), rnil()))),
                           rcell(rcell(rsym("-"), rcell(rint(10), rcell(rint(5), rnil()))), rnil()))));
}

#[test]
fn test_read_symbol() {
    assert_eq!(parse("(inc 1)").unwrap(), rcell(rsym("inc"), rcell(rint(1), rnil())));
}

#[test]
fn test_read_define() {
    assert_eq!(parse("(define x 1)").unwrap(),
               rcell(rsym("define"), rcell(rsym("x"), rcell(rint(1), rnil()))));
    assert_eq!(parse("(define x (+ 1 2))").unwrap(),
               rcell(rsym("define"), rcell(rsym("x"), rcell(rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil()))), rnil()))));
}

#[test]
fn test_read_progn() {
    assert_eq!(parse("(progn (+ 1 2) (+ 1 2))").unwrap(),
               rcell(rsym("progn"),
                     rcell(rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil()))),
                           rcell(rcell(rsym("+"), rcell(rint(1), rcell(rint(2), rnil()))), rnil()))))
}
