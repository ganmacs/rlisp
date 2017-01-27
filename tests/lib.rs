extern crate rlisp;

use rlisp::run;
use rlisp::node::*;

#[test]
fn test_run() {
    assert_eq!(run("(+ 1 2)"), Ok(rint(3)));
    assert_eq!(run("(- 100 50 10 10 10 10 10)"), Ok(rint(0)));
    assert_eq!(run("(let ((a 10)) (+ 10 a))"), Ok(rint(20)));
    assert_eq!(run("((lambda (x) x) 20)"), Ok(rint(20)));
    assert_eq!(run("((if #t (lambda (x) x) 20) 20)"), Ok(rint(20)));
    assert_eq!(run("((if #f 3 (lambda (x) x)) 20)"), Ok(rint(20)));
    assert_eq!(run("(let ((a 10)) ((lambda (x) x) a))"), Ok(rint(10)));
    assert_eq!(run("(let ((a (lambda (x) x))) (a 20))"), Ok(rint(20)));
    assert_eq!(run("(let ((c 10)) (let ((f (lambda (x) (+ x c)))) (let ((a (lambda (y) (f y)))) (a 20))))"), Ok(rint(30)));
    assert_eq!(run("((lambda (f1 f2) (f2 (f1 10) (f1 20))) (lambda (x) x) (lambda (x y) (+ x y)))"), Ok(rint(30)));
    assert_eq!(run("((if #t + -) 1 2)"), Ok(rint(3)));
}
