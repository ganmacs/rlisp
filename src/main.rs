extern crate rlisp;

fn main() {
    // match rlisp::run("(define x (+ 1 2))") {
    match rlisp::run("(progn
            (define f (lambda (x) (+ x 10)))
            (f 10)
)") {
        Ok(result) => rlisp::printer::lprint(result),
        Err(v) => println!("{:?}", v),
    }
}
