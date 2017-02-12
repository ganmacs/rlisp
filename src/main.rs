extern crate rlisp;

fn main() {
    match rlisp::run("(define x (+ 1 2))") {
        Ok(result) => rlisp::printer::lprint(result),
        Err(v) => println!("{:?}", v),
    }
}
