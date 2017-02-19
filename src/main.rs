extern crate rlisp;

fn main() {
    // let expr = "(progn (define x (+ 1 2)) (+ x 2))";
    // let expr = "(let ((x 10)
    //               (f (lambda (x) (+ x 10))))
    //             (f x))";
    // let expr = "(let ((a 10)) (+ 10 a))";
    let expr = "((lambda (f1 f2) (f2 (f1 10) (f1 20))) (lambda (x) x) (lambda (x y) (+ x y)))";

    match rlisp::run(expr) {
        Ok(result) => rlisp::printer::lprint(result),
        Err(v) => println!("{:?}", v),
    }
}
