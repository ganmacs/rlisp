extern crate rlisp;

fn main() {
    match rlisp::run("(+ 1 2 4)") {
        Ok(result) => rlisp::printer::lprint(result),
        Err(v) => println!("{:?}", v),
    }
}
