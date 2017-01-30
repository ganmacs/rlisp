extern crate rlisp;

fn main() {
    match rlisp::run("(+ 2 3 4)") {
        Ok(result) => rlisp::printer::lprint(result),
        Err(v) => println!("{:?}", v),
    }
}
