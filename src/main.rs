extern crate rlisp;

fn main() {
    // match rlisp::run("(define x (+ 1 2))") {
    // match rlisp::run("(let ((x 10)
    //                         (f (lambda (x) (+ x 10))))
    //                   (f x))") {

    match rlisp::run("((lambda (x) (+ x 1)) 1)") {
        Ok(result) => rlisp::printer::lprint(result),
        Err(v) => println!("{:?}", v),
    }
}
