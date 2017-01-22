pub mod parser;
pub mod evaluator;
pub mod printer;
use evaluator::eval;

#[derive(Debug, PartialEq)]
pub enum Node {
    Cell(Box<Node>, Box<Node>),
    Int(i32),
    Fn { name: &'static str },
    Nil,
}

pub fn run(input: &'static str) ->  Result<Node, &'static str> {    // specific type
    match parser::parse(input) {
        Ok(result) => eval(result),
        Err(v) => Err(v.to_str()),
    }
}
