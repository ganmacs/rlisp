extern crate rlisp;

fn main() {
    // let expr = "(progn (define x (+ 1 2)) (+ x 2))";
    let expr = "(let ((x 10)
                  (f (lambda (x) (+ x 10))))
                (f x))";

    match rlisp::run(expr) {
        Ok(result) => rlisp::printer::lprint(result),
        Err(v) => println!("{:?}", v),
    }
}
