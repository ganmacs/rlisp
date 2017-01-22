pub mod parser;
pub mod evaluator;
pub mod printer;
pub mod node;

use node::Node;

pub fn run(input: &'static str) ->  Result<Node, &'static str> {    // specific type
    match parser::parse(input) {
        Ok(result) => evaluator::eval(result),
        Err(v) => Err(v.to_str()),
    }
}
