extern crate rlisp;

fn main() {
    match rlisp::run("(+ 2 3 4)") {
        Ok(result) => rlisp::lprint(result),
        Err(v) => println!("{:?}", v)
    }
}

// #[cfg(test)]
// mod test {
//     #[test]
//     fn test_parsing() {
//         "(+ 2 3 4)"
//     }
// }
