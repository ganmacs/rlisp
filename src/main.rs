extern crate rlisp;

fn main() {
    match rlisp::run("(+ 1 (* 3 2))") {
        Ok(result) => rlisp::printer::lprint(result),
        Err(v) => println!("{:?}", v),
    }
}
