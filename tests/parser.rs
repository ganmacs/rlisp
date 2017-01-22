extern crate rlisp;

use rlisp::parser::parse;
use rlisp::node::Node;

#[test]
fn test_read_nil() {
    assert_eq!(parse("()").unwrap(), Node::Nil);
}


#[test]
fn test_read_int() {
    assert_eq!(parse("1").unwrap(), Node::Int(1));
    assert_eq!(parse("100").unwrap(), Node::Int(100));
    assert_eq!(parse("1000").unwrap(), Node::Int(1000));
}
